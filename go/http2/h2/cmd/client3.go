package cmd

import (
	"github.com/spf13/cobra"

	"h2/client"
)

var client3Cmd = &cobra.Command{
	Use: "client3 addr",
	Run: func(cmd *cobra.Command, args []string) {
		client.Run3(args[0])
	},
}
