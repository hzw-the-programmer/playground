package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	levels()
	setGlobalLevel()
	structured()
	logError()
	subLogger()
	fileOutput()
	prettyConsole()
}

func levels() {
	// log.Panic().Msg("This is a panic message")
	// log.Fatal().Msg("This is a fatal message")
	log.Error().Msg("This is an error message")
	log.Warn().Msg("This is a warning message")
	log.Info().Msg("This is an information message")
	log.Debug().Msg("This is a debug message")
	log.Trace().Msg("This is a trace message")
}

func setGlobalLevel() {
	zerolog.SetGlobalLevel(zerolog.DebugLevel)
	log.Debug().Msg("Debug message is displayed")
	log.Info().Msg("Info message is displayed")

	zerolog.SetGlobalLevel(zerolog.InfoLevel)
	log.Debug().Msg("Debug message is no longer displayed")
	log.Info().Msg("Info message is displayed")
}

func structured() {
	log.Info().Str("name", "hzw").Msg("master's name")
	log.Info().Int("age", 34).Msg("master's age")
	log.Info().Str("name", "hzw").Int("age", 34).Msg("master's information")
}

func logError() {
	log.Error().Err(errors.New("dial timeout")).Msg("failed to connect to remote")
}

func subLogger() {
	main := zerolog.New(os.Stderr).With().Logger()
	user := main.With().Str("module", "user").Logger()
	main.Info().Msg("begin init")
	user.Info().Msg("begin init")
	user.Info().Msg("end init")
	main.Info().Msg("end init")
}

func fileOutput() {
	f, err := ioutil.TempFile(os.TempDir(), "log")
	if err != nil {
		log.Error().Err(err).Msg("there was an error creating a temporary file for our log")
		return
	}
	main := zerolog.New(f).With().Logger()
	user := main.With().Str("module", "user").Logger()
	main.Info().Msg("begin init")
	user.Info().Msg("begin init")
	user.Info().Msg("end init")
	main.Info().Msg("end init")

	fmt.Printf("This log file is allocated at %s\n", f.Name())
}

func prettyConsole() {
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})
	log.Error().Msg("This is an error message")
	log.Warn().Msg("This is a warning message")
	log.Info().Msg("this is an information message")
	log.Debug().Msg("this is a debug message")
}
