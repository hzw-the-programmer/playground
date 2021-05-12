package loraserver

import (
	log "github.com/sirupsen/logrus"
	"iotserver/internal/config"
	"iotserver/internal/util"
	"net"
	"strings"
)

// Setup sets up lora server.
func Setup(c config.Config) error {
	log.WithFields(log.Fields{
		"bind": c.LoraServer.Bind,
	}).Info("loraserver: starting Lora Server")

	bind := strings.TrimSpace(c.LoraServer.Bind)
	if bind == "" {
		log.Info("loraserver: bind is empty")
		return nil
	}

	go startLoraServer()

	return nil
}

func startLoraServer() {
	ln, err := net.Listen("tcp", config.C.LoraServer.Bind)
	if err != nil {
		log.Info("loraserver: error:", err)
		return
	}
	defer ln.Close()

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Info("loraserver: error:", err)
			return
		}
		lc := util.NewLoraClient(conn)
		go lc.Run()
	}
}
