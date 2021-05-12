package cmd

import (
	"fmt"
	"io/ioutil"
	"log"

	"github.com/spf13/cobra"
)

var catCmd = &cobra.Command{
	Use:  "cat",
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		in, err := ioutil.ReadFile(args[0])
		if err != nil {
			log.Fatalln("Error reading file:", err)
		}

		for _, b := range in {
			fmt.Printf("0x%02x, ", b)
		}
	},
}
