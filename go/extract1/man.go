package main

import (
	"bufio"
	"io"
	"os"
	"path/filepath"
)

func main() {
	dst := os.Args[1]
	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		p := s.Text()
		copyfile(filepath.Join(dst, p), p)
	}
}

func copyfile(to, from string) {
	dir := filepath.Dir(to)
	if err := os.MkdirAll(dir, 0777); err != nil {
		panic(err)
	}
	tof, err := os.Create(to)
	if err != nil {
		panic(err)
	}
	defer tof.Close()

	fromf, err := os.Open(from)
	if err != nil {
		panic(err)
	}
	defer fromf.Close()

	io.Copy(tof, fromf)
}
