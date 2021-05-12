package main

import (
	"generate/cmd/generate/cmd"
)

var version = "unset"

func main() {
	cmd.Execute(version)
}
