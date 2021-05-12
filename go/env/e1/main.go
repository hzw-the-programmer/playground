package main

import (
	"log"
	"os"
)

// NEWVAR=haha ./e1.exe
// ./run.bat
func main() {
	os.Setenv("NAME", "hzw")
	os.Setenv("NEWVAR1", "overide")
	log.Print(os.Getenv("PATH"))
	log.Print(os.Getenv("NEWVAR"))
	log.Print(os.Getenv("NEWVAR1"))
	log.Print(os.Getenv("NAME"))
}
