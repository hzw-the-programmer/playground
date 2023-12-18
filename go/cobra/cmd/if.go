/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// ifCmd represents the if command
var ifCmd = &cobra.Command{
	Use:   "if",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: ifRun,
}

func init() {
	rootCmd.AddCommand(ifCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// ifCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// ifCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func ifRun(cmd *cobra.Command, args []string) {
	var iface If
	iface = &mockIf{}
	iface.f1()
	// iface.f2()

	iface = &mockIf{&impl1{}}
	iface.f1()
	iface.f2()

	iface = &mockIf{&impl2{}}
	iface.f1()
	iface.f2()
}

type If interface {
	f1()
	f2()
}

type mockIf struct {
	If
}

func (mi *mockIf) f1() {
	fmt.Println("f1")
}

type impl1 struct{}

func (i *impl1) f1() {
	fmt.Println("impl1 f1")
}

func (i *impl1) f2() {
	fmt.Println("impl1 f2")
}

type impl2 struct{}

func (i *impl2) f1() {
	fmt.Println("impl2 f1")
}

func (i *impl2) f2() {
	fmt.Println("impl2 f2")
}
