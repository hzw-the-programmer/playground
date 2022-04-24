/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
package cmd

import (
	//"fmt"
	"io"
	"net/http"
	"log"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"golang.org/x/net/websocket"
)

// xwebsocketCmd represents the xwebsocket command
var xwebsocketCmd = &cobra.Command{
	Use:   "xwebsocket",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		xwebsocketCmdRun()
	},
}

func init() {
	rootCmd.AddCommand(xwebsocketCmd)
	xwebsocketCmd.Flags().BoolP("client", "c", false, "run client")
	viper.BindPFlag("client", xwebsocketCmd.Flags().Lookup("client"))
	
	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// xwebsocketCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// xwebsocketCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func xwebsocketCmdRun() {
	if (viper.GetBool("client")) {
		echoClient()
	} else {
		echoServer()
	}
}

func echoServer() {
	http.Handle("/echo", websocket.Handler(echo))
	if err := http.ListenAndServe(":8000", nil); err != nil {
		log.Fatal(err)
	}
}

func echo(conn *websocket.Conn) {
	io.Copy(conn, conn)
}

func echoClient() {
	origin := "http://localhost/"
	url := "ws://localhost:8000/echo"
	conn, err := websocket.Dial(url, "", origin)
	if err != nil {
		log.Fatal(err)
	}
	if _, err := conn.Write([]byte("Hello world!")); err != nil {
		log.Fatal(err)
	}
	var msg = make([]byte, 512)
	var n int
	if n, err = conn.Read(msg); err != nil {
		log.Fatal(err)
	}
	log.Printf("received: %s", msg[:n])
}