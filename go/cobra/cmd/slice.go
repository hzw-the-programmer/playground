/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// sliceCmd represents the slice command
var sliceCmd = &cobra.Command{
	Use:   "slice",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: run_slice,
}

func init() {
	rootCmd.AddCommand(sliceCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// sliceCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// sliceCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func run_slice(cmd *cobra.Command, args []string) {
	s := make([]byte, 2, 10)
	fmt.Println(len(s), cap(s))
	fmt.Println(s[0], s[1])
	// fmt.Println(s[2])
	fmt.Println(len(s[3:5]), cap(s[3:5]))
	// fmt.Println(len(s[3:11]), cap(s[3:11]))
	fmt.Println(len(s[len(s):]), cap(s[len(s):]))
}
