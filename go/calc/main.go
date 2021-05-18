package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
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
	lis := os.Args[2]

	pat := filepath.Join(dir, "*", "*.c")
	files, err := filepath.Glob(pat)
	if err != nil {
		panic(err)
	}
	names := map[string]*Size{}
	for _, f := range files {
		base := filepath.Base(f)
		base = base[:len(base)-2] + ".obj"
		names[base] = &Size{}
	}
	pat = ""
	for n := range names {
		if len(pat) != 0 {
			pat += "|"
		}
		pat += n
	}
	pat = `\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(` + pat + ")"
	reg := regexp.MustCompile(pat)

	f, err := os.Open(lis)
	if err != nil {
		panic(err)
	}
	s := bufio.NewScanner(f)
	for s.Scan() {
		line := s.Text()
		if m := reg.FindStringSubmatch(line); m != nil {
			fmt.Println(line)
			if s, ok := names[m[7]]; ok {
				s.Code, _ = strconv.ParseInt(m[1], 10, 64)
				s.Inc, _ = strconv.ParseInt(m[2], 10, 64)
				s.RO, _ = strconv.ParseInt(m[3], 10, 64)
				s.RW, _ = strconv.ParseInt(m[4], 10, 64)
				s.ZI, _ = strconv.ParseInt(m[5], 10, 64)
				s.Debug, _ = strconv.ParseInt(m[6], 10, 64)
			}
		}
	}

	var code int64
	var inc int64
	var ro int64
	var rw int64
	var zi int64
	var debug int64
	for _, s := range names {
		code += s.Code
		inc += s.Inc
		ro += s.RO
		rw += s.RW
		zi += s.ZI
		debug += s.Debug
	}
	fmt.Println("Code total: ", code)
	fmt.Println("Inc total: ", inc)
	fmt.Println("RO total: ", ro)
	fmt.Println("RW total: ", rw)
	fmt.Println("ZI total: ", zi)
	fmt.Println("Debug total: ", debug)
}
