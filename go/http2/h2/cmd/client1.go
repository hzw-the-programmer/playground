package cmd

import (
	"github.com/spf13/cobra"

	"h2/client"
)

var client1Cmd = &cobra.Command{
	Use: "client1 addr",
	Run: func(cmd *cobra.Command, args []string) {
		client.Run1(args[0])
	},
}
