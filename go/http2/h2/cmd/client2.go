package cmd

import (
	"github.com/spf13/cobra"

	"h2/client"
)

var client2Cmd = &cobra.Command{
	Use: "client2 addr",
	Run: func(cmd *cobra.Command, args []string) {
		client.Run2(args)
	},
}
