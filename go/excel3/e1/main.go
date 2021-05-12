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

type sheets []string

func (s *sheets) String() string {
	return "sheets to generate output"
}

func (s *sheets) Set(str string) error {
	strs := strings.Split(str, ",")
	*s = append(*s, strs...)
	return nil
}

const (
	fileNameFormat = "language_%s_%d.%d"
	gzExtension    = ".gzip"
	separator      = "SePaRaToR"
)

func main() {
	var sheets sheets

	inputfile := flag.String("inputfile", "translations.xlsx", "path to traslation file")
	flag.Var(&sheets, "sheet", "sheet to generate output")
	embed := flag.Bool("embed", false, "output embed format")
	gz := flag.Bool("gzip", false, "output gzip format")
	enum := flag.Bool("enum", false, "generate enum or not")
	major := flag.Int("major", 0, "major version")
	minor := flag.Int("minor", 0, "minor version")

	flag.Parse()

	f, err := excelize.OpenFile(*inputfile)
	if err != nil {
		fmt.Println(err)
		return
	}

	var lans []string
	trans := make(map[string][]string)
	ids := make([]string, 0)

	for _, sheet := range sheets {
		rows := f.GetRows(sheet)
		if len(rows) < 2 {
			continue
		}

		if lans == nil {
			lans = rows[0][1:]
		}

		rows = rows[1:]

		ids = append(ids, separator+sheet)
		for _, row := range rows {
			id := row[0]
			trans[id] = row[1:]
			ids = append(ids, id)
		}
	}

	encoder := unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder()
	gen := func(index int, gz, embed bool) {
		lan := strings.ToLower(lans[index])

		fileName := fmt.Sprintf(fileNameFormat, lan, *major, *minor)

		if embed {
			fileName = "embed_" + fileName
		}

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

		if embed {
			hex := true
			if !gz && lan == "english" {
				hex = false
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
			if strings.HasPrefix(id, separator) {
				continue
			}
			tran := trans[id]
			text := ""
			if index < len(tran) && len(tran[index]) > 0 {
				text = tran[index]
			} else if len(tran) != 0 {
				text = tran[0]
			}

			writer.Write([]byte(text))
			writer.Write([]byte{0})
		}
	}

	for i, lan := range lans {
		gen(i, *gz, *embed || lan == "english")
	}

	if *enum {
		fileName := fmt.Sprintf("enum_%d.%d", *major, *minor)

		file, err := os.Create(fileName)
		if err != nil {
			log.Fatal(err)
		}
		defer file.Close()

		for _, id := range ids {
			if strings.HasPrefix(id, separator) {
				fmt.Fprintf(file, "// %s,\n", id[len(separator):])
			} else {
				fmt.Fprintf(file, "%s,\n", id)
			}
		}
	}
}
