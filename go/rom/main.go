package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	// "sort"
	"strconv"
	"strings"
)

var reg1 = regexp.MustCompile(`\s+(?P<Code>\d+)\s+(?P<inc>\d+)\s+(?P<RO>\d+)\s+(?P<RW>\d+)\s+(?P<ZI>\d+)\s+(?P<Debug>\d+)\s+(?P<name>[\w.-]+\.\w+)`)

// var reg2 = regexp.MustCompile(`  ([\w-.]+\.obj)\(letschat\.lib\)`)

func main() {
	if len(os.Args) != 2 {
		log.Fatalf("Usage: %s filepath", os.Args[0])
	}
	filepath := os.Args[1]
	file, err := os.Open(filepath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// kv := make(map[string]int)
	// scanner := bufio.NewScanner(file)
	// for scanner.Scan() {
	// 	m := reg2.FindSubmatch(scanner.Bytes())
	// 	if m == nil {
	// 		continue
	// 	}
	// 	k := string(m[1])
	// 	_, ok := kv[k]
	// 	if !ok {
	// 		kv[k] = 1
	// 	} else {
	// 		kv[k]++
	// 	}
	// }

	// var ks []string
	// for k, _ := range kv {
	// 	ks = append(ks, k)
	// }
	// sort.Strings(ks)
	// for _, v := range ks {
	// 	log.Print(v)
	// }

	totalCode := 0
	totalInc := 0
	totalRo := 0
	totalRw := 0
	totalZi := 0
	totalDebug := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		m := reg1.FindSubmatch(scanner.Bytes())
		if m == nil {
			continue
		}

		code, _ := strconv.Atoi(string(m[1]))
		inc, _ := strconv.Atoi(string(m[2]))
		ro, _ := strconv.Atoi(string(m[3]))
		rw, _ := strconv.Atoi(string(m[4]))
		zi, _ := strconv.Atoi(string(m[5]))
		debug, _ := strconv.Atoi(string(m[6]))

		name := string(m[7])
		if strings.HasPrefix(name, "ra_") ||
			strings.HasPrefix(name, "letschat_") ||
			strings.Contains(name, "pb-c") ||
			strings.HasPrefix(name, "picohttpparser") ||
			strings.HasPrefix(name, "protobuf-c") {
			log.Printf("%s, %d, %d, %d, %d, %d, %d", name, code, inc, ro, rw, zi, debug)
			totalCode += code
			totalInc += inc
			totalRo += ro
			totalRw += rw
			totalZi += zi
			totalDebug += debug
		}
	}

	log.Printf("%d, %d, %d, %d, %d, %d", totalCode, totalInc, totalRo, totalRw, totalZi, totalDebug)
}
