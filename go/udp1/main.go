package main

import (
	"encoding/hex"
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"sync"
	"sync/atomic"

	"golang.org/x/net/websocket"

	"github.com/hzw-the-programmer/python-play-ground/go/protocol/ptest"

	"github.com/hzw-the-programmer/python-play-ground/go/udp1/client"
	"github.com/hzw-the-programmer/python-play-ground/go/udp1/utils"
)

type udppkt struct {
	addr net.Addr
	data []byte
}

type server struct {
	udpConn     net.PacketConn
	udpSendChan chan udppkt

	wg sync.WaitGroup

	m   sync.Mutex
	seq uint32
}

var srv = server{}

func main() {
	isClient := flag.Bool("ic", false, "is client")
	udpAddr := flag.String("ua", ":2311", "udp address")
	httpAddr := flag.String("ha", ":1123", "http address")

	flag.Parse()

	log.SetFlags(log.LstdFlags | log.Llongfile)

	if *isClient {
		client.Client(*udpAddr)
	} else {
		serve(*udpAddr, *httpAddr)
	}
}

func serve(udpAddr, httpAddr string) {
	srv.wg.Add(1)
	go serveUdp(udpAddr)

	srv.wg.Add(1)
	go serveHttp(httpAddr)

	srv.wg.Wait()
}

func serveUdp(addr string) {
	defer srv.wg.Done()

	conn, err := net.ListenPacket("udp", addr)
	if err != nil {
		utils.LogWaitExit(err)
	}
	srv.udpConn = conn
	log.Printf("listen packet at: %s", conn.LocalAddr())

	srv.wg.Add(1)
	go readpkt()

	srv.udpSendChan = make(chan udppkt)

	srv.wg.Add(1)
	go sendpkt()
}

func serveHttp(addr string) {
	defer srv.wg.Done()

	http.Handle("/", http.FileServer(http.Dir("web")))
	http.Handle("/ws", websocket.Handler(ws))

	err := http.ListenAndServe(addr, nil)
	log.Print(err)
}

func readpkt() {
	defer srv.wg.Done()

	buf := make([]byte, 1024)
	for {
		n, addr, err := srv.udpConn.ReadFrom(buf)
		if err != nil {
			utils.LogWaitExit(err)
		}
		log.Printf("read from %s: %s", addr, hex.EncodeToString(buf[:n]))
	}
}

func sendpkt() {
	defer srv.wg.Done()

	for pkt := range srv.udpSendChan {
		n, err := srv.udpConn.WriteTo(pkt.data, pkt.addr)
		if err != nil {
			log.Printf("sendpkt: write to err: %s", err)
			break
		}
		log.Printf("write to %s: %s", pkt.addr, hex.EncodeToString(pkt.data[:n]))
	}
}

func ws(conn *websocket.Conn) {
	for {
		var data map[string]interface{}

		err := websocket.JSON.Receive(conn, &data)
		if err != nil {
			log.Printf("ws: json receive err: %s", err)
			break
		}

		switch data["cmd"].(string) {
		case "ping":
			host := data["host"].(string)
			port := data["port"].(string)
			addrstr := fmt.Sprintf("%s:%s", host, port)
			log.Printf("ws: ping: %s", addrstr)
			addr, err := net.ResolveUDPAddr("", addrstr)
			if err != nil {
				log.Printf("ws: ping: wrong addr: %s", err)
				continue
			}

			seq := atomic.AddUint32(&srv.seq, 1)

			data, err := ptest.Ping{Seq: seq}.MarshalBinary()
			if err != nil {
				log.Printf("ws: ping: marshal err: %s", err)
				continue
			}
			srv.udpSendChan <- udppkt{addr, data}
		}
	}
}
