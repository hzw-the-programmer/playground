/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	// "fmt"
	"log"
	"net"

	"github.com/spf13/cobra"
)

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
		addr := ""
		if len(args) > 0 {
			addr = args[0]
		}
		
		conn, err := net.ListenPacket("udp", "")
		if err != nil {
			log.Fatal(err)
		}
		defer conn.Close()

		log.Printf("local address: %s\n", conn.LocalAddr())

		if addr != "" {
			client(conn, addr);
		} else {
			server(conn);
		}
	},
}

func init() {
	rootCmd.AddCommand(udp1Cmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// udp1Cmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// udp1Cmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func client(conn net.PacketConn, _addr string) {
	addr, err := net.ResolveUDPAddr("udp", _addr)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(addr)
	msg := "hello world"
	conn.WriteTo([]byte(msg), addr)
}

func server(conn net.PacketConn) {
	var buf [1024]byte
	for {
		n, addr, err := conn.ReadFrom(buf[:])
		if err != nil {
			log.Fatal(err)
		}
		log.Printf("%s: %s\n", addr, buf[:n])
	}
}