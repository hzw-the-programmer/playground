package main

// go build -o build/main
// GOOS=windows GOARCH=amd64 go build -o build/main.exe

import (
	"bufio"
	"encoding/base64"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"os"
	"path"
	"regexp"
	"strconv"
	"strings"
	"sync"
	"time"
)

var cmd *string

var r *regexp.Regexp

func main() {
	cmd = flag.String("cmd", "multi", "multi, single, join, filter")
	cf := flag.String("cf", "config.txt", "config file store sns")
	lf := flag.String("lf", "yymmdd.log", "log file to process")
	flag.Parse()

	r = regexp.MustCompile(`Recv: (.*), (.*), (.*)`)

	if *cmd == "multi" || *cmd == "single" {
		processDir(".", ".log", convertLog)
	} else if *cmd == "filter" {
		filter(*cf, *lf)
	} else {
		processDir(".", ".log", genJoin)
	}
}

func processDir(dir, ext string, processFile func(fn string)) {
	d, err := os.Open(dir)
	if err != nil {
		log.Printf("processDir(%s, %s): os.Open error: %s", dir, ext, err)
		return
	}

	fis, err := d.Readdir(-1)
	if err != nil {
		log.Printf("processDir(%s, %s): Readdir error: %s", dir, ext, err)
		return
	}

	start := time.Now()

	var wg sync.WaitGroup
	for _, fi := range fis {
		fn := fi.Name()
		if fi.IsDir() || path.Ext(fn) != ext {
			continue
		}
		wg.Add(1)
		go func(fn string) {
			processFile(fn)
			wg.Done()
		}(fn)
	}
	wg.Wait()

	elapsed := time.Now().Sub(start)
	log.Printf("processDir(%s, %s): elapsed: %s", dir, ext, elapsed)
}

func convertLog(fn string) {
	dn := strings.TrimSuffix(fn, path.Ext(fn))
	if *cmd == "single" {
		dn += ".csv"
	}
	_, err := os.Stat(dn)
	if err == nil {
		log.Printf("process(%s): %s exists", fn, dn)
		return
	}

	f, err := os.Open(fn)
	if err != nil {
		log.Printf("process(%s): os.Open error: %s", fn, err)
		return
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	for s.Scan() {
		m := r.FindStringSubmatch(s.Text())

		if len(m) < 4 {
			continue
		}
		t := m[1]
		gw := m[2]
		js := m[3]

		var j map[string]interface{}
		err = json.Unmarshal([]byte(js), &j)
		if err != nil {
			log.Printf("process(%s): json.Unmarshal error: %s", fn, err)
			continue
		}

		rxpks, ok := j["rxpk"].([]interface{})
		if !ok {
			continue
		}
		for i := range rxpks {
			rxpk := rxpks[i].(map[string]interface{})

			bs, err := base64.StdEncoding.DecodeString(rxpk["data"].(string))
			if err != nil {
				log.Printf("process(%s): base64.StdEncoding.DecodeString error: %s", fn, err)
				continue
			}

			if len(bs) < 6 {
				continue
			}

			i0 := bs[0]
			i1 := bs[1]
			i2 := bs[2]
			i3 := bs[3]
			i4 := bs[4]
			i5 := bs[5]

			if i0 != 0x00 && i0 != 0x40 && i0 != 0x80 {
				continue
			}

			remark := "-1"
			pb := 6
			if i0 == 0x00 {
				if i3&0x10 == 0x10 {
					remark = "request"
				} else {
					remark = "join"
				}
			} else if i0 == 0x40 {
				if i3&0x0f == 0x03 {
					if len(bs) < 9 {
						continue
					}
					i6 := bs[6]
					i7 := bs[7]
					i8 := bs[8]
					pb = 9
					if i6 == 0x01 {
						remark = strconv.Itoa(int(i7) | (int(i8) << 8))
					}
				}
			}

			appid := (i1 & 0x0c) >> 2
			devaddr := (uint16(((i1>>4)<<2)|(i1&0x03)) << 8) | uint16(i2)
			fcnt := uint16(i4) | (uint16(i5) << 8)

			bs = bs[pb:]

			if len(bs) < 12 {
				continue
			}
			sn := fmt.Sprintf("%014d", binary.BigEndian.Uint64(bs[4:]))

			freq := rxpk["freq"].(float64)
			stat := rxpk["stat"].(float64)
			lsnr := rxpk["lsnr"].(float64)
			rssi := rxpk["rssi"].(float64)

			if *cmd == "single" {
				writeFile(
					dn, sn, t, gw,
					appid, devaddr, fcnt,
					remark,
					freq, stat, lsnr, rssi,
					bs)
			} else {
				mkdirAndWriteFile(
					dn, sn, t, gw,
					appid, devaddr, fcnt,
					remark,
					freq, stat, lsnr, rssi,
					bs)
			}
		}
	}

	log.Printf("process(%s): finish", fn)
}

func mkdirAndWriteFile(
	dn string, sn string, t string, gw string,
	appid uint8, devaddr uint16, fcnt uint16,
	remark string,
	freq float64, stat float64, lsnr float64, rssi float64,
	bs []byte) bool {

	err := os.MkdirAll(dn, 0777)
	if err != nil {
		log.Fatal(err)
	}

	// err = os.Chdir(dn)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	writeFile(
		path.Join(dn, sn+".csv"), sn, t, gw,
		appid, devaddr, fcnt,
		remark,
		freq, stat, lsnr, rssi,
		bs)

	// err = os.Chdir("..")
	// if err != nil {
	// 	log.Fatal(err)
	// }

	return true
}

func writeFile(
	fn string, sn string, t string, gw string,
	appid uint8, devaddr uint16, fcnt uint16,
	remark string,
	freq float64, stat float64, lsnr float64, rssi float64,
	bs []byte) bool {

	_, err := os.Stat(fn)
	ne := err != nil

	f, err := os.OpenFile(fn, os.O_WRONLY|os.O_CREATE|os.O_APPEND, 0666)
	if err != nil {
		log.Print("OpenFile error: ", err)
		return false
	}
	defer f.Close()

	if ne {
		_, err = fmt.Fprint(f, "sn, time, gateway, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, payload\n")
		if err != nil {
			log.Print("write header err: ", err)
			return false
		}
	}

	_, err = fmt.Fprintf(f,
		"%s, %s, %s, %d, %d, %d, %s, %.1f, %.0f, %.1f, %.1f, %s\n",
		sn, t, gw, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, hex.EncodeToString(bs))
	if err != nil {
		log.Print("write field err: ", err)
		return false
	}

	return true
}

func scanFile(fn string, bs, as func(), pl func(line string)) error {
	f, err := os.Open(fn)
	if err != nil {
		return err
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	if bs != nil {
		bs()
	}
	if pl != nil {
		for s.Scan() {
			pl(s.Text())
		}
	}
	if as != nil {
		as()
	}

	return nil
}

func genJoin(fn string) {
	fn = strings.TrimSuffix(fn, path.Ext(fn)) + ".csv"
	m := make(map[string]map[string]*RecvInfo)

	pl := func(line string) {
		fields := strings.Split(line, ", ")

		sn := fields[0]
		gateway := fields[2]
		remark := fields[6]
		stat, _ := strconv.Atoi(fields[8])

		if remark == "request" || remark == "join" || stat == -1 {
			return
		}

		if m[sn] == nil {
			m[sn] = make(map[string]*RecvInfo)
		}
		m[sn][gateway] = nil
	}
	if err := scanFile(fn, nil, nil, pl); err != nil {
		log.Fatal(err)
	}

	fno := strings.TrimSuffix(fn, path.Ext(fn)) + "_join.csv"
	fo, err := os.Create(fno)
	if err != nil {
		log.Fatal(err)
	}
	defer fo.Close()
	_, err = fmt.Fprint(fo, "sn, time, gateway, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, payload\n")
	if err != nil {
		log.Fatal(err)
	}
	pl1 := func(line string) {
		fields := strings.Split(line, ", ")

		sn := fields[0]
		time := fields[1]
		gateway := fields[2]
		appid := fields[3]
		devaddr := fields[4]
		fcnt := fields[5]
		remark := fields[6]
		freq, _ := strconv.ParseFloat(fields[7], 64)
		stat, _ := strconv.Atoi(fields[8])
		lsnr, _ := strconv.ParseFloat(fields[9], 64)
		rssi, _ := strconv.ParseFloat(fields[10], 64)
		payload := fields[11]

		if remark != "request" && remark != "join" {
			return
		}
		if stat == -1 {
			return
		}

		if _, ok := m[sn]; !ok {
			return
		}
		if _, ok := m[sn][gateway]; !ok {
			return
		}
		m[sn][gateway] = &RecvInfo{
			Sn:      sn,
			Time:    time,
			Gateway: gateway,
			Appid:   appid,
			DevAddr: devaddr,
			Fcnt:    fcnt,
			Remark:  remark,
			Freq:    freq,
			Stat:    stat,
			Lsnr:    lsnr,
			Rssi:    rssi,
			Payload: payload,
		}
	}
	as1 := func() {
		for _, v1 := range m {
			for _, v2 := range v1 {
				if v2 != nil {
					_, err = fmt.Fprintf(fo,
						"%s, %s, %s, %s, %s, %s, %s, %.1f, %d, %.1f, %.1f, %s\n",
						v2.Sn, v2.Time, v2.Gateway, v2.Appid, v2.DevAddr, v2.Fcnt, v2.Remark, v2.Freq, v2.Stat, v2.Lsnr, v2.Rssi, v2.Payload)
					if err != nil {
						continue
					}
				}
			}
		}
	}
	scanFile(fn, nil, as1, pl1)
}

func parseFile(fn string, cb func(sn, t, gw string,
	appid uint8, devaddr uint16, fcnt uint16, remark string,
	freq float64, stat float64, lsnr float64, rssi float64, bs []byte)) {
	f, err := os.Open(fn)
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	for s.Scan() {
		m := r.FindStringSubmatch(s.Text())

		if len(m) < 4 {
			continue
		}
		t := m[1]
		gw := m[2]
		js := m[3]

		var j map[string]interface{}
		err = json.Unmarshal([]byte(js), &j)
		if err != nil {
			log.Print(err)
			continue
		}

		rxpks, ok := j["rxpk"].([]interface{})
		if !ok {
			continue
		}
		for i := range rxpks {
			rxpk := rxpks[i].(map[string]interface{})

			bs, err := base64.StdEncoding.DecodeString(rxpk["data"].(string))
			if err != nil {
				log.Print(err)
				continue
			}

			if len(bs) < 6 {
				continue
			}

			i0 := bs[0]
			i1 := bs[1]
			i2 := bs[2]
			i3 := bs[3]
			i4 := bs[4]
			i5 := bs[5]

			if i0 != 0x00 && i0 != 0x40 && i0 != 0x80 {
				continue
			}

			remark := "-1"
			pb := 6
			if i0 == 0x00 {
				if i3&0x10 == 0x10 {
					remark = "request"
				} else {
					remark = "join"
				}
			} else if i0 == 0x40 {
				if i3&0x0f == 0x03 {
					if len(bs) < 9 {
						continue
					}
					i6 := bs[6]
					i7 := bs[7]
					i8 := bs[8]
					pb = 9
					if i6 == 0x01 {
						remark = strconv.Itoa(int(i7) | (int(i8) << 8))
					}
				}
			}

			appid := (i1 & 0x0c) >> 2
			devaddr := (uint16(((i1>>4)<<2)|(i1&0x03)) << 8) | uint16(i2)
			fcnt := uint16(i4) | (uint16(i5) << 8)

			bs = bs[pb:]

			if len(bs) < 12 {
				continue
			}
			sn := fmt.Sprintf("%014d", binary.BigEndian.Uint64(bs[4:]))

			freq := rxpk["freq"].(float64)
			stat := rxpk["stat"].(float64)
			lsnr := rxpk["lsnr"].(float64)
			rssi := rxpk["rssi"].(float64)

			cb(sn, t, gw, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, bs)
		}
	}
}

func filter(cf, lf string) {
	d := strings.TrimSuffix(path.Base(lf), path.Ext(lf))
	err := os.MkdirAll(d, 0777)
	if err != nil {
		log.Fatal(err)
	}

	cff, err := os.Open(cf)
	if err != nil {
		log.Fatal(err)
	}
	defer cff.Close()

	sns := make(map[string]*os.File)
	s := bufio.NewScanner(cff)
	for s.Scan() {
		sn := s.Text()
		f, err := os.Create(path.Join(d, sn+".csv"))
		if err != nil {
			log.Print(err)
			continue
		}
		_, err = fmt.Fprint(f, "sn, time, gateway, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, payload\n")
		if err != nil {
			log.Print(err)
			f.Close()
			continue
		}
		sns[sn] = f
	}

	parseFile(lf, func(sn, t, gw string,
		appid uint8, devaddr uint16, fcnt uint16, remark string,
		freq float64, stat float64, lsnr float64, rssi float64, bs []byte) {
		f, ok := sns[sn]
		if !ok {
			return
		}
		_, err = fmt.Fprintf(f,
			"%s, %s, %s, %d, %d, %d, %s, %.1f, %.0f, %.1f, %.1f, %s\n",
			sn, t, gw, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, hex.EncodeToString(bs))
		if err != nil {
			log.Print(err)
		}
	})

	for _, f := range sns {
		f.Close()
	}
}

type RecvInfo struct {
	Sn      string
	Time    string
	Gateway string
	Appid   string
	DevAddr string
	Fcnt    string
	Remark  string
	Freq    float64
	Stat    int
	Lsnr    float64
	Rssi    float64
	Payload string
}
