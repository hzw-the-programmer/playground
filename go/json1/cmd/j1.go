package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/spf13/cobra"
)

type Message struct {
	Name string
	Body string
	Time int64
}

type FamilyMember struct {
	Name    string
	Age     int
	Parents []string
}

type Bar struct {
	With   int64
	Height int64
}

type Foo struct {
	Name string
	Bar  *Bar
	Age  int64
}

var j1Cmd = &cobra.Command{
	Use: "j1",
	Run: func(cmd *cobra.Command, args []string) {
		j1()
	},
}

func j1() {
	m := Message{
		Name: "Alice",
		Body: "Hello",
		Time: 1294706395881547000,
	}
	b, err := json.Marshal(m)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(b))

	var m1 Message
	err = json.Unmarshal(b, &m1)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", m1)

	b = []byte(`{"Name":"Bob","Food":"Pickle"}`)
	err = json.Unmarshal(b, &m1)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", m1)

	b = []byte(`{"Name":"Wednesday","Age":6,"Parents":["Gomez","Morticia"]}`)
	var f interface{}
	err = json.Unmarshal(b, &f)
	m2 := f.(map[string]interface{})
	for k, v := range m2 {
		switch vv := v.(type) {
		case string:
			fmt.Println(k, "is string", vv)
		case float64:
			fmt.Println(k, "is float64", vv)
		case []interface{}:
			fmt.Println(k, "is an array:")
			for i, u := range vv {
				fmt.Println(i, u)
			}
		default:
			fmt.Println(k, "is of a type I don't know how to handle")
		}
	}

	var m3 FamilyMember
	err = json.Unmarshal(b, &m3)
	fmt.Printf("%+v\n", m3)

	b = []byte(`{"Name":"Wednesday","Age":6}`)
	var m4 FamilyMember
	err = json.Unmarshal(b, &m4)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", m4)

	b = []byte(`{"Name": "hzw", "Age": 35, "Bar": {"With": 128, "Height": 160}}`)
	var foo Foo
	err = json.Unmarshal(b, &foo)
	fmt.Printf("%+v\n", foo)
	fmt.Printf("%+v\n", foo.Bar)

	b = []byte(`{"Name": "hzw", "Age": 35}`)
	var foo1 Foo
	err = json.Unmarshal(b, &foo1)
	fmt.Printf("%+v\n", foo1)
	fmt.Printf("%+v\n", foo1.Bar)
}
