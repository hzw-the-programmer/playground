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

// go run main.go t2 fixture/t2 > out
// t2Cmd represents the t2 command
var t2Cmd = &cobra.Command{
	Use:   "t2",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		fn := args[0]
		pat := regexp.MustCompile(`\[(\d+)\]\s*=\s*(\d+)\s*,`)
		var max int
		vals := map[int]int{}

		f, err := os.Open(fn)
		if err != nil {
			log.Fatal(err)
		}
		defer f.Close()

		s := bufio.NewScanner(f)
		for s.Scan() {
			line := s.Text()
			if m := pat.FindAllStringSubmatch(line, -1); m != nil {
				for i := 0; i < len(m); i++ {
					index, err := strconv.Atoi(m[i][1])
					if err != nil {
						log.Fatal(err)
					}
					val, err := strconv.Atoi(m[i][2])
					if err != nil {
						log.Fatal(err)
					}
					vals[index] = val
					if index > max {
						max = index
					}
				}
			}
		}

		// fmt.Println(vals)
		// fmt.Println(len(vals))
		// fmt.Println(max)

		for i := 0; i <= max; i++ {
			val := vals[i]
			str := fmt.Sprintf("%d,", val)
			fmt.Printf("%-3s // %d\n", str, i)
		}
	},
}

func init() {
	rootCmd.AddCommand(t2Cmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// t2Cmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// t2Cmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
