package lora

import (
	"encoding/json"
	"log"
	"net"
)

func Serve(addr string) {
	lis, err := net.Listen("tcp", addr)
	if err != nil {
		log.Fatal(err)
	}

	for {
		conn, err := lis.Accept()
		if err != nil {
			log.Fatal(err)
		}
		lc := NewConn(conn)

		go func() {
			for {
				bd, err := lc.ReadBd()
				if err != nil {
					log.Print(err)
					break
				}
				log.Print(string(bd))

				var j Proto
				err = json.Unmarshal(bd, &j)
				if err != nil {
					log.Print(err)
					break
				}
				if j.Cmd == "join" {
					bd = []byte(`{"cmd":"join_ack","cmdseq":1,"appeui":"4B46415050000003","code":200,"msg":"JOIN ACCEPT"}`)

				}

				err = lc.WriteBd(bd)
				if err != nil {
					log.Print(err)
					break
				}
			}
		}()
	}
}
