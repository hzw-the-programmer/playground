package main

import (
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"sync"
	"time"
)

func runningtime(s string) (string, time.Time) {
	log.Print("Start: ", s)
	return s, time.Now()
}

func track(s string, start time.Time) {
	log.Print("End: ", s, " took ", time.Now().Sub(start))
}

func main() {
	defer track(runningtime("download"))

	pname := flag.String("name", "", "")
	pid := flag.Int("id", 0, "")
	ppic := flag.String("pic", "", "")
	preferer := flag.String("referer", "", "")
	pdir := flag.String("dir", "", "")
	pext := flag.String("ext", "jpg", "")
	pnum := flag.Int("num", 100, "")

	flag.Parse()

	if *pname == "" || *pid == 0 || *ppic == "" || *preferer == "" || *pdir == "" {
		fmt.Printf("Usage: %s -name=? -id=? -pic=? -referer=? -dir=?\n", os.Args[0])
		return
	}

	dir := fmt.Sprintf("%s/%s/%d", *pdir, *pname, *pid)

	err := os.MkdirAll(dir, 0777)
	if err != nil {
		log.Fatal(err)
	}

	var wg sync.WaitGroup

	for i := 1; i < *pnum; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()

			pic := fmt.Sprintf("%s/%d/%d.%s", *ppic, *pid, i, *pext)
			referer := fmt.Sprintf("%s/%d_%d.html", *preferer, *pid, i)

			req, err := http.NewRequest("GET", pic, nil)
			if err != nil {
				log.Print(err)
				return
			}
			req.Header.Set("Referer", referer)

			resp, err := http.DefaultClient.Do(req)
			if err != nil {
				log.Print(err)
				return
			}
			defer resp.Body.Close()

			if resp.StatusCode != 200 {
				log.Print(resp.StatusCode)
				return
			}

			file, err := os.Create(fmt.Sprintf("%s/%d.%s", dir, i, *pext))
			if err != nil {
				log.Print(err)
				return
			}
			defer file.Close()

			io.Copy(file, resp.Body)
		}(i)
	}

	wg.Wait()
}
