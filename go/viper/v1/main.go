package main

import (
	"bytes"
	"fmt"

	"github.com/spf13/viper"
)

func main() {
	var yamlExample = []byte(`
Hacker: true
name: steve
hobbies:
- skateboarding
- snowboarding
- go
clothing:
  jacket: leather
  trousers: denim
age: 35
eyes : brown
beard: true
`)

	viper.SetConfigType("yml")
	viper.ReadConfig(bytes.NewBuffer(yamlExample))

	viper.Debug()
	fmt.Println(viper.Get("name"))
	fmt.Println(viper.Get("clothing.jacket"))
	fmt.Println(viper.Get("clothing.trousers"))
	fmt.Println(viper.GetStringSlice("hobbies")[0])
	fmt.Printf("%#v\n", viper.Get("hobbies"))
	fmt.Printf("%#v\n", viper.Get("clothing"))
}
