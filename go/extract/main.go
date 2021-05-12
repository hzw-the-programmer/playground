package main

import (
	"bufio"
	"io"
	"log"
	"os"
	"regexp"
)

var (
	macroExp = regexp.MustCompile(`---- (\w+) Matches \(\d+ in \d+ files\) ----`)
	pathExp  = regexp.MustCompile(`(\w+\.?\w*) \(([\w\\]+)\).* : `)
)

func copy(dstPath, srcPath string) {
	dst, err := os.Create(dstPath)
	if err != nil {
		log.Fatal(err)
	}
	defer dst.Close()

	src, err := os.Open(srcPath)
	if err != nil {
		log.Fatal(err)
	}
	defer src.Close()

	_, err = io.Copy(dst, src)
	if err != nil {
		log.Fatal(err)
	}

}

func main() {
	if len(os.Args) != 3 {
		log.Fatalf("Usage: %s searchResultFile projectDir", os.Args[0])
	}

	searchResultFile := os.Args[1]
	projectDir := os.Args[2]

	if projectDir[len(projectDir)-1] != '\\' || projectDir[len(projectDir)-1] != '/' {
		projectDir += "/"
	}

	file, err := os.Open(searchResultFile)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var macro string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()

		matches := macroExp.FindStringSubmatch(text)
		if matches != nil {
			macro = matches[1]
			os.RemoveAll(macro)
			continue
		}

		matches = pathExp.FindStringSubmatch(text)
		if matches != nil {
			srcPath := projectDir + matches[2] + "/" + matches[1]
			dstPath := macro + "/" + matches[2]
			if _, err := os.Stat(dstPath); err != nil {
				if err := os.MkdirAll(dstPath, 0666); err != nil {
					log.Fatal(err)
				}
			}
			dstPath += "/" + matches[1]
			copy(dstPath, srcPath)
		}
	}
}
