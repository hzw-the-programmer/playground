package cmd

import (
	"os"
	"os/signal"
	"syscall"

	"github.com/pkg/errors"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"

	"iotserver/internal/config"
	"iotserver/internal/loraclient"
	"iotserver/internal/loraserver"
)

func rootRun(cmd *cobra.Command, args []string) error {
	tasks := []func() error{
		setLogLevel,
		printStartMessage,
		setupLoraServer,
		setupLoraClient,
	}

	for _, task := range tasks {
		if err := task(); err != nil {
			log.Fatal(err)
		}
	}

	sigChan := make(chan os.Signal)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	log.WithField("signal", <-sigChan).Info("signal received")
	log.Warning("shutting down server")

	return nil
}

func setLogLevel() error {
	log.SetLevel(log.Level(config.C.General.LogLevel))
	return nil
}

func printStartMessage() error {
	log.WithFields(log.Fields{
		"version": version,
		"docs":    "constructing",
	}).Info("cmd: starting Iot Server")
	return nil
}

func setupLoraServer() error {
	if err := loraserver.Setup(config.C); err != nil {
		return errors.Wrap(err, "setup Lora Server error")
	}
	return nil
}

func setupLoraClient() error {
	if err := loraclient.Setup(config.C); err != nil {
		return errors.Wrap(err, "setup Lora Client error")
	}
	return nil
}
