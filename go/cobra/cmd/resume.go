/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"log"
	"os"

	"github.com/spf13/cobra"
)

// resumeCmd represents the resume command
var resumeCmd = &cobra.Command{
	Use:   "resume",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: resumeRun,
}

func init() {
	rootCmd.AddCommand(resumeCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// resumeCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	resumeCmd.Flags().BoolVar(&encrypt, "enc", true, "encrypt or decrypt")
}

var encrypt bool

func resumeRun(cmd *cobra.Command, args []string) {
	key := []byte{0x4e, 0xa0, 0x55, 0x1e, 0xe7, 0x78, 0x6e, 0xec, 0xec, 0xf2, 0x86, 0x11, 0x0, 0xff, 0xa7, 0xc6, 0x70, 0x16, 0x79, 0xb8, 0x6f, 0x88, 0x49, 0x76, 0x1d, 0x99, 0x4c, 0xc1, 0xd5, 0xb, 0x41, 0x92}
	iv := []byte{0xc8, 0x59, 0x36, 0xb8, 0xce, 0xca, 0xb0, 0xa1, 0x47, 0x70, 0x54, 0x52, 0xf8, 0x9e, 0x78, 0x13}

	b, err := aes.NewCipher(key)
	if err != nil {
		log.Fatal(err)
	}

	var bm cipher.BlockMode
	var in string
	var out string
	if encrypt {
		bm = cipher.NewCBCEncrypter(b, iv)
		in = "resume.md"
		out = "resume"
	} else {
		bm = cipher.NewCBCDecrypter(b, iv)
		in = "resume"
		out = "resume.md"
	}

	content, err := os.ReadFile(in)
	if err != nil {
		log.Fatal(err)
	}

	if encrypt {
		paddingLen := b.BlockSize() - len(content)%b.BlockSize()
		content = append(content, bytes.Repeat([]byte{byte(paddingLen)}, paddingLen)...)
	}

	bm.CryptBlocks(content, content)

	if !encrypt {
		paddingLen := int(content[len(content)-1])
		content = content[:len(content)-paddingLen]
	}

	err = os.WriteFile(out, content, 0666)
	if err != nil {
		log.Fatal(err)
	}
}
