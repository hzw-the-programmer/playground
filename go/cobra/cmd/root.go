package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "t1",
	Short: "t1 short",
	Long: `t1 long
		   multiline`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Say hello to hzw.")
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
