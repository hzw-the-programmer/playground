package cmd

import (
	"github.com/spf13/cobra"

	"h2/client"
)

var clientCmd = &cobra.Command{
	Use:   "client [addr to connect]",
	Short: "Run the client",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		client.Run(args[0])
	},
}
