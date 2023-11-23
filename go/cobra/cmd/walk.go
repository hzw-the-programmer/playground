/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/spf13/cobra"
)

// walkCmd represents the walk command
var walkCmd = &cobra.Command{
	Use:   "walk",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		// filepath.WalkDir(args[0], func(path string, d fs.DirEntry, err error) error {
		// 	fmt.Println(path)
		// })
	},
}

func init() {
	rootCmd.AddCommand(walkCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// walkCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// walkCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
