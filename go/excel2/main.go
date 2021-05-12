package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/360EntSecGroup-Skylar/excelize"
	"golang.org/x/text/encoding/unicode"
)

func main() {
	exlFile := excelize.NewFile()

	txtFile, err := os.Open("translations.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer txtFile.Close()

	decoder := unicode.UTF16(unicode.LittleEndian, unicode.UseBOM).NewDecoder()
	reader := decoder.Reader(txtFile)
	scanner := bufio.NewScanner(reader)
	row := 0
	for scanner.Scan() {
		fields := strings.Split(scanner.Text(), "\t")
		row++
		for i := 0; i < len(fields); i++ {
			cellName := fmt.Sprintf("%c%d", 'A'+i, row)
			exlFile.SetCellValue("Sheet1", cellName, fields[i])
		}
	}

	if err := exlFile.SaveAs("translations.xlsx"); err != nil {
		log.Fatal(err)
	}
}
