/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"log"
	"bytes"
	// "crypto/rand"
	"crypto/aes"
	"crypto/cipher"

	"github.com/spf13/cobra"
)

// aesCmd represents the aes command
var aesCmd = &cobra.Command{
	Use:   "aes",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println(args)

		// var key [32]byte
		// if n, err := rand.Read(key[:]); err != nil || n != 32 {
		// 	log.Fatal(err)
		// }
		// log.Printf("%#v", key)

		// var iv [16]byte
		// if n, err := rand.Read(iv[:]); err != nil || n != 16 {
		// 	log.Fatal(err)
		// }
		// log.Printf("%#v", iv)

		key := []byte{0x4e, 0xa0, 0x55, 0x1e, 0xe7, 0x78, 0x6e, 0xec, 0xec, 0xf2, 0x86, 0x11, 0x0, 0xff, 0xa7, 0xc6, 0x70, 0x16, 0x79, 0xb8, 0x6f, 0x88, 0x49, 0x76, 0x1d, 0x99, 0x4c, 0xc1, 0xd5, 0xb, 0x41, 0x92}
		block, err := aes.NewCipher(key)
		if err != nil {
			log.Fatal(err)
		}
		iv := []byte{0xc8, 0x59, 0x36, 0xb8, 0xce, 0xca, 0xb0, 0xa1, 0x47, 0x70, 0x54, 0x52, 0xf8, 0x9e, 0x78, 0x13}
		enc := cipher.NewCBCEncrypter(block, iv)
		dec := cipher.NewCBCDecrypter(block, iv)

		// src := "0"
		// src := "012345678901234"
		// src := "0123456789012345"
		// src := "01234567890123456"
		// src := "01234567890123456789";
		src := "01234567890123456789012345678901";
		// src := "0123456789012345678901234567890123456789012";
		padding_len := 16 - len(src) % 16
		// if padding_len == 16 {
		// 	padding_len = 0
		// }
		log.Printf("padding_len=%d", padding_len)
		dst := append([]byte(src), bytes.Repeat([]byte{byte(padding_len)}, padding_len)...)
		log.Printf("%#v", dst)
		
		enc.CryptBlocks(dst[:], dst[:])
		log.Printf("%#v", dst)

		dec.CryptBlocks(dst[:], dst[:])
		log.Print(string(dst[:]))
	},
}

func init() {
	rootCmd.AddCommand(aesCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	aesCmd.PersistentFlags().Int32P("key_size", "k", 256, "key size")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// aesCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
