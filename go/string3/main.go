package main

import (
	"compress/gzip"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"strings"

	"github.com/360EntSecGroup-Skylar/excelize"
	"golang.org/x/text/encoding/unicode"
)

const fileNameFormat = "language_%s_%d.%d"
const gzExtension = ".gzip"

func main() {
	major := flag.Int("major", 0, "major version number")
	minor := flag.Int("minor", 1, "minor version bumber")

	flag.Parse()

	transFileName := "translations.xlsx"
	if flag.NArg() > 0 {
		transFileName = flag.Arg(0)
	}

	transFile, err := excelize.OpenFile(transFileName)
	if err != nil {
		log.Fatal(err)
	}

	lans := make([]string, 0)
	trans := make(map[string][]string)
	ids := make([]string, 0)

	rows := transFile.GetRows("Sheet1")
	for _, row := range rows {
		if row[0] == "id" {
			lans = row[1:]
		} else {
			id := row[0]
			ids = append(ids, id)
			trans[id] = row[1:]
		}
	}

	inputLans := lans
	if flag.NArg() > 1 {
		inputLans = flag.Args()[1:]
	}

	inputLansIndex := make([]int, len(inputLans))
	for i, inputLan := range inputLans {
		j := 0
		for j < len(lans) {
			if inputLan == lans[j] {
				inputLansIndex[i] = j
				break
			}
			j++
		}
		if j == len(lans) {
			log.Fatalf("%s does not exist", inputLan)
		}
	}

	englishIndex := -1
	for i, lan := range lans {
		if lan == "english" {
			englishIndex = i
			break
		}
	}
	if englishIndex == -1 {
		log.Fatal("no english translations")
	}

	encoder := unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder()
	gen := func(index int, gz bool) {
		lan := strings.ToLower(lans[index])

		fileName := fmt.Sprintf(fileNameFormat, lan, *major, *minor)
		if gz {
			fileName += gzExtension
		}

		file, err := os.Create(fileName)
		if err != nil {
			log.Fatal(err)
		}
		defer file.Close()

		var writer io.Writer

		writer = file

		if lan == "english" {
			hex := false
			if gz {
				hex = true
			}
			writer = &ArrayWriter{file, hex, 0}
		}

		if gz {
			zw := gzip.NewWriter(writer)
			defer zw.Close()
			writer = zw
		}

		if lan != "english" {
			writer = encoder.Writer(writer)
		}

		for _, id := range ids {
			tran := trans[id]
			text := ""
			if index < len(tran) && len(tran[index]) > 0 {
				text = tran[index]
			} else if englishIndex < len(tran) {
				text = tran[englishIndex]
			}

			writer.Write([]byte(text))
			writer.Write([]byte{0})
		}
	}

	for _, index := range inputLansIndex {
		gen(index, false)
		gen(index, true)
	}
}
