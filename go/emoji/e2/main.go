package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	f, err := os.Open("./emoji.json")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	d, err := ioutil.ReadAll(f)
	if err != nil {
		panic(err)
	}

	var emojis map[string]interface{}
	err = json.Unmarshal(d, &emojis)
	if err != nil {
		panic(err)
	}

	var emojisGrp [][]string
	count := 0
	for k, _ := range emojis {
		ks := strings.Split(k, "-")
		ksLen := len(ks)
		emojisGrpLen := len(emojisGrp)

		if ksLen == 6 {
			count++
		}

		for i := emojisGrpLen; i < ksLen; i++ {
			emojisGrp = append(emojisGrp, []string{})
		}
		emojisGrp[ksLen-1] = append(emojisGrp[ksLen-1], k)
	}

	fmt.Printf("count=%d\n", count)

	for k, v := range emojisGrp {
		fmt.Println(k, len(v))
	}
}
