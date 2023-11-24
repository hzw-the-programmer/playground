/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"reflect"

	"github.com/spf13/cobra"
)

// reflectCmd represents the reflect command
var reflectCmd = &cobra.Command{
	Use:   "reflect",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: runReflect,
}

func init() {
	rootCmd.AddCommand(reflectCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// reflectCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// reflectCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

type bar struct {
	x int
}

type Foo struct {
	A int
	a bar `t1:"v1" t2:"v2"`
}

func runReflect(cmd *cobra.Command, args []string) {
	var foo Foo

	typ := reflect.TypeOf(foo)

	fmt.Println(typ.Name())
	fmt.Println(typ.Kind())
	fmt.Println(typ.NumField())
	fmt.Println(typ.Field(0).Name)
	fmt.Println(typ.Field(0).Type.Name())
	fmt.Println(typ.Field(0).Type.Kind())

	fmt.Println(typ.Field(1).Name)
	fmt.Println(typ.Field(1).Type.Name())
	fmt.Println(typ.Field(1).Type.Kind())
	fmt.Println(typ.Field(1).Tag)
}
