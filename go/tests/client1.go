package main

import (
	"fmt"
	"log"
	"net"
	"time"
)

func main() {
	for {
		conn, err := net.Dial("tcp", "localhost:3210")
		if err != nil {
			log.Println(err)
			time.Sleep(1 * time.Second)
			continue
		}

		recvMsgCh := make(chan []byte)
		sendMsgCh := make(chan []byte)
		quitCh := make(chan error, 1)

		go recvMsgs(conn, recvMsgCh, quitCh)
		go sendMsgs(conn, sendMsgCh, quitCh)
		go makeMsgs(sendMsgCh)

		<-quitCh

		conn.Close()
		close(recvMsgCh)
		close(sendMsgCh)
		close(quitCh)
	}
}

func recvMsgs(conn net.Conn, recvMsgCh chan<- []byte, quitCh chan<- error) {
	buf := make([]byte, 512)
	for {
		n, err := conn.Read(buf)
		if err != nil {
			log.Println(err)
			quitCh <- err
			break
		}
		fmt.Println(string(buf[:n]))
	}
	fmt.Println("recvMsgs: leave")
}

func sendMsgs(conn net.Conn, sendMsgCh <-chan []byte, quitCh chan<- error) {
	for msg := range sendMsgCh {
		_, err := conn.Write(msg)
		if err != nil {
			log.Println(err)
			quitCh <- err
			break
		}
	}
	fmt.Println("sendMsgs: leave")
}

func makeMsgs(msgChan chan<- []byte) {
	defer func() {
		fmt.Println("makeMsgs: leave")
		recover()
	}()
	for {
		time.Sleep(1 * time.Second)
		msgChan <- []byte("hello go")
	}
}
