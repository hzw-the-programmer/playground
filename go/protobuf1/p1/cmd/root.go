package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{}

func init() {
	rootCmd.AddCommand(addCmd, listCmd, catCmd)
	rootCmd.AddCommand(t1Cmd)
}

func Execute() {
	rootCmd.Execute()
}
