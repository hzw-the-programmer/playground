package lora

import (
	"encoding/json"
	"log"
	"net"
	"sync"
	"time"
)

func Connect(addr string) {
	var wg sync.WaitGroup
	readch := make(chan []byte)
	writech := make(chan []byte)
	send := func(bd []byte) bool {
		select {
		case writech <- bd:
			return true
		default:
			return false
		}
	}

	go func() {
		for {
			time.Sleep(1 * time.Second)

			log.Print("dial")
			conn, err := net.Dial("tcp", addr)
			if err != nil {
				log.Print(err)
				continue
			}
			lc := NewConn(conn)

			stopch := make(chan struct{})

			wg.Add(1)
			go func() {
				defer func() {
					close(stopch)
					wg.Done()
				}()

				first := true
				for {
					bd, err := lc.ReadBd()
					if err != nil {
						log.Print(err)
						break
					}
					if first {
						var j Proto
						err := json.Unmarshal(bd, &j)
						if err != nil {
							log.Print(err)
							break
						}
						if j.Msg != "JOIN ACCEPT" {
							log.Print("join failed")
							break
						}
						first = false
					} else {
						readch <- bd
					}
				}
			}()

			wg.Add(1)
			go func() {
				defer wg.Done()

				var jn = []byte("{\"cmd\": \"join\", \"cmdseq\": 1, \"appeui\": \"4B46415050000003\"}")
				err := lc.WriteBd(jn)
				if err != nil {
					log.Print(err)
					return
				}

				for {
					select {
					case bd := <-writech:
						err := lc.WriteBd(bd)
						if err != nil {
							log.Print(err)
							return
						}
					case <-stopch:
						return
					}
				}
			}()

			wg.Wait()
			conn.Close()
		}
	}()

	go func() {
		hb := []byte("{\"cmd\": \"heartbeat\", \"appeui\": \"4B46415050000003\"}")
		for {
			select {
			case <-time.Tick(1 * time.Second):
				send(hb)
			}
		}
	}()

	for bd := range readch {
		log.Print(string(bd))
	}
}
