package client

import (
	"context"
	"io/ioutil"
	"log"
	"net"
	"net/http"
	"time"
)

// ./h2.exe client2 "www.baidu.com/sleep" "localhost:8081"

func Run2(addres []string) {
	cli := &http.Client{
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				log.Println(network, addr)
				dialer := net.Dialer{
					// Timeout: 500 * time.Millisecond,
				}
				start := time.Now()
				conn, err := dialer.Dial("tcp", addres[1])
				// conn, err := dialer.DialContext(ctx, "tcp", addres[1])
				log.Println("dial:", time.Now().Sub(start), err)
				return conn, err
			},
		},
		Timeout: 1 * time.Second,
	}

	req, err := http.NewRequest(http.MethodGet, "http://"+addres[0], nil)
	if err != nil {
		log.Fatal(err)
	}

	start := time.Now()
	res, err := cli.Do(req)
	log.Print(time.Now().Sub(start))
	if err != nil {
		log.Print(err)
		time.Sleep(3 * time.Second)
		log.Fatal("")
	}
	defer res.Body.Close()

	buf, err := ioutil.ReadAll(res.Body)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(string(buf))
}
