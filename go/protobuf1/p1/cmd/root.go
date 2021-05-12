package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{}

func init() {
	rootCmd.AddCommand(addCmd, listCmd, catCmd)
}

func Execute() {
	rootCmd.Execute()
}
