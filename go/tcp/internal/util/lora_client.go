package util

import (
	log "github.com/sirupsen/logrus"
	"net"
	"time"
)

type LoraClient struct {
	ls        *LoraStream
	recvMsgCh chan []byte
	sendMsgCh chan []byte
	quitCh    chan error
}

func NewLoraClient(conn net.Conn) *LoraClient {
	lc := &LoraClient{
		ls:        NewLoraStream(conn, conn),
		recvMsgCh: make(chan []byte),
		sendMsgCh: make(chan []byte),
		quitCh:    make(chan error, 1),
	}
	return lc
}

func (lc *LoraClient) GetRecvMsgCh() chan []byte {
	return lc.recvMsgCh
}

func (lc *LoraClient) GetSendMsgCh() chan []byte {
	return lc.sendMsgCh
}

func (lc *LoraClient) Run() {
	go lc.recvMsgs()
	go lc.sendMsgs()
	ticker := time.NewTicker(1 * time.Second)
	for stop := false; !stop; {
		select {
		case <-ticker.C:
			lc.sendMsg([]byte("hello hzw"))
		case <-lc.quitCh:
			ticker.Stop()
			lc.stop()
			stop = true
		}
	}
}

func (lc *LoraClient) recvMsgs() {
	for {
		msg, err := lc.ls.ReadMsg()
		if err != nil {
			log.Info(err)
			lc.quitCh <- err
			break
		}
		log.Info(string(msg))
		//lc.recvMsgCh <- msg
	}
	log.Info("LoraClient: leave recvMsgs")
}

func (lc *LoraClient) sendMsgs() {
	for msg := range lc.sendMsgCh {
		err := lc.ls.WriteMsg(msg)
		if err != nil {
			log.Info(err)
			lc.quitCh <- err
			break
		}
	}
	log.Info("LoraClient: leave sendMsgs")
}

func (lc *LoraClient) stop() {
	close(lc.sendMsgCh)
}

func (lc *LoraClient) sendMsg(msg []byte) {
	lc.sendMsgCh <- msg
}
