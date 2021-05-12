package client

import (
	"encoding/hex"
	"log"
	"net"
	"time"

	"github.com/hzw-the-programmer/python-play-ground/go/protocol/p4"
	"github.com/hzw-the-programmer/python-play-ground/go/protocol/ptest"

	"github.com/hzw-the-programmer/python-play-ground/go/udp1/utils"
)

func Client(addr string) {
	conn, err := net.Dial("udp", addr)
	if err != nil {
		utils.LogWaitExit(err)
	}
	log.Printf("addr: %s", conn.LocalAddr())

	go readpkt(conn)

	pkt := p4.Packet{
		Header: &p4.Header{
			From: 0x54,
			Ver:  0x34,
			Len:  49,
			Sn:   20170123000001,
			Seq:  0x12345678,
			Time: time.Date(2018, 01, 23, 23, 11, 11, 0, time.Local),
		},
		Body: &p4.HeartBeatWithData{
			Seq: 0x78563412,
			Channels: []p4.Channel{
				{
					Slot:    1,
					Port:    0,
					Type:    p4.ChanTypeTemp,
					Status:  0x02,
					Data:    40.3,
					AvgData: 41.3,
				},
				{
					Slot:    2,
					Port:    0,
					Type:    p4.ChanTypeHumi,
					Status:  0x00,
					Data:    79.3,
					AvgData: 80.3,
				},
			},
		},
	}
	for {
		select {
		case <-time.Tick(1 * time.Second):
			bs, err := pkt.Marshal()
			if err != nil {
				utils.LogWaitExit(err)
			}
			conn.Write(bs)
		}
	}
}

func readpkt(conn net.Conn) {
	data := make([]byte, 1024)
	for {
		n, err := conn.Read(data)
		if err != nil {
			log.Fatal("conn read error: %s", err)
		}
		log.Print(hex.EncodeToString(data[:n]))

		var p ptest.Ping
		err = p.UnmarshalBinary(data[:n])
		if err == nil {
			reply, err := ptest.Ack{Cmd: ptest.PING, Seq: p.Seq}.MarshalBinary()
			if err != nil {
				log.Printf("ack marshal error: %s", err)
				continue
			}
			conn.Write(reply)
		}
	}
}
