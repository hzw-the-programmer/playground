package main

import (
	"iotserver/cmd/iotserver/cmd"
)

var version string

func main() {
	cmd.Execute(version)
}
