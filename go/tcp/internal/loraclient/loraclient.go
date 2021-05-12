package loraclient

import (
	log "github.com/sirupsen/logrus"
	"iotserver/internal/config"
	"iotserver/internal/util"
	"net"
	"strings"
	"time"
)

// Setup sets up lora client.
func Setup(_ config.Config) error {
	log.WithFields(log.Fields{
		"address": config.C.LoraServer.Address,
	}).Info("loraclient: starting Lora Client")

	address := strings.TrimSpace(config.C.LoraServer.Address)
	if address == "" {
		log.Info("loraclient: address is empty")
		return nil
	}

	go startLoraClient()

	return nil
}

func startLoraClient() {
	for {
		conn, err := net.Dial("tcp", config.C.LoraServer.Address)
		if err != nil {
			log.Info(err)
			time.Sleep(1 * time.Second)
			continue
		}

		lc := util.NewLoraClient(conn)
		lc.Run()
	}
}
