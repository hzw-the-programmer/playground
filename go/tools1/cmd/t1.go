/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"

	"github.com/spf13/cobra"
)

// go run main.go t1 fixture/t1 > out
// t1Cmd represents the t1 command
var t1Cmd = &cobra.Command{
	Use:   "t1",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		fn := args[0]
		pat := regexp.MustCompile(`\s*\[I\((\d+),\s*(\d+)\)\]\s*=\s*(\{\S*\})`)
		var i, j int

		f, err := os.Open(fn)
		if err != nil {
			log.Fatal(err)
		}
		defer f.Close()

		s := bufio.NewScanner(f)
		for s.Scan() {
			line := s.Text()
			if m := pat.FindStringSubmatch(line); m != nil {
				l, err := strconv.Atoi(m[1])
				if err != nil {
					log.Fatal(err)
				}
				h, err := strconv.Atoi(m[2])
				if err != nil {
					log.Fatal(err)
				}
				if l != i || h != j {
					log.Fatalf("want (%d, %d), got (%d, %d)", i, j, l, h)
				}
				fmt.Printf("%-20s // (%d, %d) = %d\n", m[3]+",", l, h, l+(h<<8))
				i += 1
				if i == 256 {
					i = 0
					j += 1
				}
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(t1Cmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// t1Cmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// t1Cmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
