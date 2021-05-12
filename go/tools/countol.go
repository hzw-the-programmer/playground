package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	"strconv"
)

// "=""QSMC                ""","=""F7                  ""","=""East                ""","=""L27                 ""","=""检测GPU               ""","=""02111191300001""",手圈,"2019-09-23 23:29:50","2019-09-24 00:00:00",1810,STANDBY,128.42MΩ

var p = regexp.MustCompile(`"(\d+)""",(.*),"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})","(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})",(\d+),(\w+)`)

func main() {
	stats := make(map[string]map[int]int)
	interval := 600
	
	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	r := bufio.NewReader(f)
	for {
		line, _, err := r.ReadLine()
		if err != nil {
			log.Println(err)
			break
		}
		matches := p.FindStringSubmatch(string(line))
		if len(matches) == 0 {
			continue
		}
		sn := matches[1]
		//t := matches[2]
		//sdt := matches[3]
		//edt := matches[4]
		duration, err := strconv.Atoi(matches[5])
		if err != nil {
			continue
		}
		//status := matches[6]
		_, ok := stats[sn]
		if !ok {
			stats[sn] = make(map[int]int)
		}
		stats[sn][duration/interval]++
		log.Println(stats)
	}
}
