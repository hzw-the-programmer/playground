package cmd

import (
	"github.com/spf13/cobra"

	"h2/server"
)

var serverCmd = &cobra.Command{
	Use:   "server [port to listen]",
	Short: "Run the server",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		server.Run(args[0])
	},
}
