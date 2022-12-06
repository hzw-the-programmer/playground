/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	// "fmt"
	"bufio"
	"log"
	"net"
	"os"

	"github.com/spf13/cobra"
)

// go mod init n2
// cobra-cli init
// cobra-cli add udp1
// go run main.go udp1 --port 6789
// go run main.go udp1 --port 6790 localhost:6789
// go fmt ./...
// GOOS=windows GOARCH=amd64 go build

// modis ok
// go run main.go udp1 --port 6789 --rcvlen 25152
// modis not ok
// go run main.go udp1 --port 6789 --rcvlen 25153

// udp1Cmd represents the udp1 command
var udp1Cmd = &cobra.Command{
	Use:   "udp1",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		log.Println("rcvlen =", rcvlen)
		addr := ""
		if len(args) > 0 {
			addr = args[0]
		}

		conn, err := net.ListenPacket("udp", ":"+port)
		if err != nil {
			log.Fatal(err)
		}
		defer conn.Close()

		log.Printf("local address: %s\n", conn.LocalAddr())

		if addr != "" {
			client(conn, addr)
		} else {
			server(conn)
		}
	},
}

var port string
var rcvlen int

func init() {
	rootCmd.AddCommand(udp1Cmd)
	udp1Cmd.Flags().StringVar(&port, "port", "", "local port")
	udp1Cmd.Flags().IntVar(&rcvlen, "rcvlen", 1024, "rcvlen")

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// udp1Cmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// udp1Cmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func recv(conn net.PacketConn) {
	buf := make([]byte, rcvlen)
	for {
		n, addr, err := conn.ReadFrom(buf[:])
		if err != nil {
			log.Fatal(err)
		}
		log.Println(addr, n)
		log.Printf("%s\n", buf[:n])
	}
}

func client(conn net.PacketConn, addrstr string) {
	go recv(conn)

	addr, err := net.ResolveUDPAddr("udp", addrstr)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(addr)

	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		_, err := conn.WriteTo(s.Bytes(), addr)
		if err != nil {
			log.Fatal(err)
		}
	}
}

func server(conn net.PacketConn) {
	buf := make([]byte, rcvlen)
	for {
		n, addr, err := conn.ReadFrom(buf[:])
		if err != nil {
			log.Fatal(err)
		}
		log.Println(addr, n)
		//log.Printf("%s\n", buf[:n])
		conn.WriteTo(buf[:n], addr)
	}
}
