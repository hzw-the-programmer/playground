package main

import (
	"bytes"
	"log"
	"os"
	"os/exec"
	"path/filepath"
)

func lib() {
	dir := os.Args[1]
	dir = filepath.Join(dir, "*.obj")

	files, err := filepath.Glob(dir)
	if err != nil {
		log.Fatal(err)
	}

	path, err := exec.LookPath("lib")
	if err != nil {
		log.Fatal(err)
	}

	args := append([]string{"lib"}, files...)
	args = append(args, "/out:app.lib")

	cmd := exec.Cmd{Path: path, Args: args}
	var out bytes.Buffer
	cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		log.Fatal(err)
	}
	log.Print(out.String())
}

func armar() {
	dir := os.Args[2]
	dir = filepath.Join(dir, "*.obj")

	files, err := filepath.Glob(dir)
	if err != nil {
		log.Fatal(err)
	}

	path, err := exec.LookPath("armar")
	if err != nil {
		log.Fatal(err)
	}

	args := append([]string{"armar"}, "-r", "app.a")
	args = append(args, files...)

	cmd := exec.Cmd{Path: path, Args: args}
	var out bytes.Buffer
	cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		log.Fatal(err)
	}
	log.Print(out.String())
}

func main() {
	lib()
	armar()
}
