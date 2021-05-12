package utils

import (
	"bufio"
	"log"
	"os"
)

func LogWaitExit(msg interface{}) {
	log.Print(msg)
	bufio.NewReader(os.Stdin).ReadLine()
	os.Exit(1)
}
