package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"time"
)

type Size struct {
	Code  int64
	Inc   int64
	RO    int64
	RW    int64
	ZI    int64
	Debug int64
}

func main() {
	if len(os.Args) != 3 {
		fmt.Printf("Usage: %s dir mapfile\n", os.Args[0])
		return
	}

	dir := os.Args[1]
	mapfile := os.Args[2]

	sizes := map[string]*Size{}
	err := filepath.Walk(dir, func(path string, f os.FileInfo, err error) error {
		if filepath.Ext(path) == ".c" {
			base := filepath.Base(path)
			base = base[:len(base)-2]
			sizes[base] = &Size{}
		}
		return nil
	})
	if err != nil {
		panic(err)
	}

	pat := ""
	for name := range sizes {
		if len(pat) != 0 {
			pat += "|"
		}
		pat += name
	}
	pat = `\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(` + pat + `)(\.\S+)`
	reg := regexp.MustCompile(pat)

	out, err := os.Create(time.Now().Format("20060102150405") + ".size")
	if err != nil {
		panic(err)
	}
	defer out.Close()

	in, err := os.Open(mapfile)
	if err != nil {
		panic(err)
	}
	defer in.Close()

	scanner := bufio.NewScanner(in)
	for scanner.Scan() {
		line := scanner.Text()
		if m := reg.FindStringSubmatch(line); m != nil {
			fmt.Fprintln(out, line)
			if s, ok := sizes[m[7]]; ok {
				var err error
				s.Code, err = strconv.ParseInt(m[1], 10, 64)
				if err != nil {
					panic(err)
				}
				s.Inc, err = strconv.ParseInt(m[2], 10, 64)
				if err != nil {
					panic(err)
				}
				s.RO, err = strconv.ParseInt(m[3], 10, 64)
				if err != nil {
					panic(err)
				}
				s.RW, err = strconv.ParseInt(m[4], 10, 64)
				if err != nil {
					panic(err)
				}
				s.ZI, err = strconv.ParseInt(m[5], 10, 64)
				if err != nil {
					panic(err)
				}
				s.Debug, err = strconv.ParseInt(m[6], 10, 64)
				if err != nil {
					panic(err)
				}
			}
		}
	}

	var code int64
	var inc int64
	var ro int64
	var rw int64
	var zi int64
	var debug int64
	for _, s := range sizes {
		code += s.Code
		inc += s.Inc
		ro += s.RO
		rw += s.RW
		zi += s.ZI
		debug += s.Debug
	}
	fmt.Fprintln(out, "Code: ", code)
	fmt.Fprintln(out, "(inc. data): ", inc)
	fmt.Fprintln(out, "RO Data: ", ro)
	fmt.Fprintln(out, "RW Data: ", rw)
	fmt.Fprintln(out, "ZI Data: ", zi)
	fmt.Fprintln(out, "Debug: ", debug)
	fmt.Fprintln(out, "Total RO Size (Code +  RO Data): ", code+ro)
	fmt.Fprintln(out, "Total RW Size (RW Data + ZI Data): ", rw+zi)
	fmt.Fprintln(out, "Total ROM Size (Code + RO Data + RW Data): ", code+ro+rw)
}
