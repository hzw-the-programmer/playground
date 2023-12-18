/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"bufio"
	"io"
	"log"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
)

// rmNonAsciiCmd represents the rmNonAscii command
var rmNonAsciiCmd = &cobra.Command{
	Use:   "rmNonAscii",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: rmNonAsciiRun,
}

func init() {
	rootCmd.AddCommand(rmNonAsciiCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// rmNonAsciiCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// rmNonAsciiCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func rmNonAsciiRun(cmd *cobra.Command, args []string) {
	_, err := rmNonAscii(args[0])
	if err != nil {
		log.Fatal(err)
	}

}

func rmNonAscii(path string) (tempPath string, err error) {
	src, err := os.OpenFile(path, os.O_RDWR, 0)
	if err != nil {
		return
	}
	defer src.Close()

	br := bufio.NewReader(src)

	dst, err := os.CreateTemp("", filepath.Base(path))
	if err != nil {
		return
	}

	bw := bufio.NewWriter(dst)

	defer func() {
		bw.Flush()
		if err == nil {
			src.Seek(0, io.SeekStart)
			dst.Seek(0, io.SeekStart)
			src.Truncate(0)
			io.Copy(src, dst)
		}
		dst.Close()
		tempPath = dst.Name()
		os.Remove(tempPath)
	}()

	for {
		r, l, e := br.ReadRune()
		if e != nil {
			log.Print(e)
			if e != io.EOF {
				err = e
			}
			break
		}

		if l > 1 {
			continue
		}

		bw.WriteRune(r)
	}

	return
}
