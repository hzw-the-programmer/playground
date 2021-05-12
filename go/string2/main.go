package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"

	"golang.org/x/text/encoding/unicode"
)

func main() {
	if len(os.Args) < 4 {
		log.Fatalf("Usage: %s id trans out [lan...]", os.Args[0])
	}

	idFileName := os.Args[1]
	idFile, err := os.Open(idFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer idFile.Close()

	ids := make([]string, 0)
	scanner := bufio.NewScanner(idFile)
	for scanner.Scan() {
		ids = append(ids, scanner.Text())
	}

	transFileName := os.Args[2]
	transFile, err := os.Open(transFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer transFile.Close()

	numOfLan := 0
	var lans []string
	trans := make(map[string][]string)
	decoder := unicode.UTF16(unicode.LittleEndian, unicode.UseBOM).NewDecoder()
	reader := decoder.Reader(transFile)
	scanner = bufio.NewScanner(reader)
	for scanner.Scan() {
		text := scanner.Text()
		fields := strings.Split(text, "\t")
		if fields[0] == "Number of Language" {
			numOfLan, err = strconv.Atoi(fields[1])
			if err != nil {
				log.Fatal(err)
			}
		} else if fields[0] == "Enum Value" {
			lans = fields[4:]
			lans = lans[:numOfLan]
		} else if fields[0] == "DO NOT MODIFY" || fields[0] == "##number" {
		} else {
			id := fields[0]
			if _, ok := trans[id]; ok {
				log.Fatalf("duplicated: %s", id)
			}
			trans[id] = fields[4:]
		}
	}

	outFileName := os.Args[3]
	outFile, err := os.Create(outFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer outFile.Close()

	inLans := os.Args[4:]
	if len(inLans) == 0 {
		inLans = lans
	}

	inLansIndex := make([]int, len(inLans))
	for i, lan := range inLans {
		j := 0
		for j < len(lans) {
			if strings.ToLower(lan) == strings.ToLower(lans[j]) {
				inLansIndex[i] = j
				break
			}
			j++
		}
		if j == len(lans) {
			log.Fatalf("%s does not exist", lan)
		}
	}

	encoder := unicode.UTF16(unicode.LittleEndian, unicode.UseBOM).NewEncoder()
	writer := encoder.Writer(outFile)
	writer.Write([]byte("id\t"))
	for i, lan := range inLans {
		sep := "\t"
		if i == len(inLans)-1 {
			sep = "\n"
		}
		writer.Write([]byte(strings.ToLower(lan)))
		writer.Write([]byte(sep))
	}

	for _, id := range ids {
		id = strings.Replace(id, "LC_STR", "STR_LC", 1)
		writer.Write([]byte(id))
		writer.Write([]byte("\t"))
		for i, li := range inLansIndex {
			sep := "\t"
			if i == len(inLansIndex)-1 {
				sep = "\n"
			}
			if li < len(trans[id]) {
				writer.Write([]byte(trans[id][li]))
			}
			writer.Write([]byte(sep))
		}
	}
}
