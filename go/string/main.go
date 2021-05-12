package main

import (
	"bufio"
	//"crypto/md5"
	"log"
	"os"
	"strconv"
	"strings"

	"golang.org/x/text/encoding/unicode"
)

func main() {
	if len(os.Args) < 3 {
		log.Fatalf("Usage: %s idFile transFile [language...]", os.Args[0])
	}

	idFileName := os.Args[1]
	transFileName := os.Args[2]
	var inputLanguage []string
	if len(os.Args) > 3 {
		inputLanguage = os.Args[3:]
	}

	idFile, err := os.Open(idFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer idFile.Close()

	var ids []string
	scanner := bufio.NewScanner(idFile)
	for scanner.Scan() {
		ids = append(ids, scanner.Text())
	}

	transFile, err := os.Open(transFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer transFile.Close()

	decoder := unicode.UTF16(unicode.LittleEndian, unicode.UseBOM).NewDecoder()
	scanner = bufio.NewScanner(decoder.Reader(transFile))

	numOfLanguage := 0
	var language []string
	strs := map[string][]string{}

	for scanner.Scan() {
		text := scanner.Text()
		fields := strings.Split(text, "\t")

		if fields[0] == "Number of Language" {
			nol, err := strconv.Atoi(fields[1])
			if err != nil {
				log.Fatal(err)
			}
			numOfLanguage = nol
		} else if fields[0] == "Enum Value" {
			l := fields[4:]
			if len(l) < numOfLanguage {
				log.Fatalf("number of language < %d", numOfLanguage)
			}
			language = l[:numOfLanguage]
		} else if fields[0] != "DO NOT MODIFY" && fields[0] != "##number" {
			id, trans := fields[0], fields[4:]
			if _, ok := strs[id]; ok {
				log.Fatalf("%s exists", id)
			}
			l := len(trans)
			if l < numOfLanguage {
				log.Printf("%s: trans is %d", id, l)
			} else {
				l = numOfLanguage
			}
			strs[id] = trans[:l]
		}
	}

	encoder := unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder()
	//h := md5.New()

	if inputLanguage == nil {
		inputLanguage = language
	}

	/*
		for _, inputLan := range inputLanguage {
			file, err := os.Open(inputLan)
			if err != nil {
				log.Fatal(err)
			}
			defer file.Close()

			len, err := file.Seek(0, 2)
			if err != nil {
				log.Fatal(err)
			}
			len -= 18
			b := make([]byte, 1024)
			h := md5.New()
			file.Seek(2, 0)

			for len > 0 {
				l := 0

				if len > 1024 {
					l = 1024
				} else {
					l = int(len)
				}

				l, _ = file.Read(b[:l])
				h.Write(b[:l])

				len -= int64(l)
			}
			md51 := [16]byte{}
			file.Seek(-16, 2)
			file.Read(md51[:])
			//log.Print(md51)
			md52 := h.Sum(nil)
			for i, b := range md51 {
				if b != md52[i] {
					log.Fatalf("%s: md5 wrong", inputLan)
				}
			}
		}
		os.Exit(0)
	*/

	for _, inputLan := range inputLanguage {
		index := -1
		for i, lan := range language {
			if inputLan == lan {
				index = i
				break
			}
		}

		if index == -1 {
			log.Fatal("language does not exist")
		}

		file, err := os.Create(inputLan)
		if err != nil {
			log.Fatal(err)
		}
		defer file.Close()

		//h.Reset()

		w := encoder.Writer(file)
		/*
		if inputLan == "English" {
			w = file
		}
		*/
		//wh := encoder.Writer(h)

		//file.Write([]byte{byte(len(ids)), byte(len(ids) >> 8)})

		for _, id := range ids {
			trans := strs[id]

			str := ""
			if index < len(trans) {
				str = trans[index]
			}
			if str == "" {
				log.Printf("%s is empty string", id)
			}
			w.Write([]byte(str))
			w.Write([]byte{0})
			//wh.Write([]byte(str))
			//wh.Write([]byte{0})
		}

		//file.Write(h.Sum(nil))
	}
}
