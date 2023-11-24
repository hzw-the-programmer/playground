/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"reflect"
	"strings"

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
	X int
}

type Foo struct {
	A int
	a bar `t1:"v1" t2:"v2"`
	B bar
}

func runReflect(cmd *cobra.Command, args []string) {
	// 1
	s := []int{1, 2, 3}
	st := reflect.TypeOf(s)
	examiner(st, 0)

	var foo Foo
	fooT := reflect.TypeOf(foo)
	examiner(fooT, 0)

	pFoo := &foo
	pFooT := reflect.TypeOf(pFoo)
	examiner(pFooT, 0)

	str := "hello"
	strT := reflect.TypeOf(str)
	examiner(strT, 0)

	p := &str
	pT := reflect.TypeOf(p)
	examiner(pT, 0)

	// 2
	g := "hello"
	gpt := reflect.ValueOf(&g)
	gpt.Elem().SetString("world")
	fmt.Println(g)

	// 3
	fooV := reflect.New(fooT)
	fooV.Elem().Field(0).SetInt(123)
	newf := fooV.Elem().Interface().(Foo)
	fmt.Println(newf)
	// fooV.Elem().Field(1).Field(0).SetInt(11)
	// fmt.Println(newf)
	// fooV.Elem().Field(2).Field(0).SetInt(11)
	// fmt.Println(newf)
	fooV.Elem().Field(2).Field(1).SetInt(11)
	fmt.Println(newf)
	newf = fooV.Elem().Interface().(Foo)
	fmt.Println(newf)
	// fooV.Elem().Field(1).Field(1).SetInt(11)
	// newf = fooV.Elem().Interface().(Foo)
	// fmt.Println(newf)
}

func examiner(t reflect.Type, depth int) {
	fmt.Printf("%sType is %q, Kind is %q\n", strings.Repeat("\t", depth), t.Name(), t.Kind())
	switch t.Kind() {
	case reflect.Slice, reflect.Ptr:
		fmt.Printf("%sContained type:\n", strings.Repeat("\t", depth+1))
		examiner(t.Elem(), depth+1)
	case reflect.Struct:
		for i := 0; i < t.NumField(); i++ {
			f := t.Field(i)
			fmt.Printf("%sField%d, Name:%q, Type:%q, Kind:%q, Tag:%s\n", strings.Repeat("\t", depth+1), i+1, f.Name, f.Type.Name(), f.Type.Kind(), f.Tag)
		}
	}
}
