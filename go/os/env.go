package main

import (
	"log"
	"os"
)

func m(key string) string {
	switch key {
	case "USER":
		return "hzw"
	case "GOPROXY":
		return "hehe"
	}
	return ""
}

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)
	for _, s := range os.Environ() {
		log.Print(s)
	}
	log.Print(os.ExpandEnv("当前用户为：$USER"))
	log.Print(os.ExpandEnv("我们使用$GOPROXY作为GO代理"))
	log.Print(os.ExpandEnv("haha${LANG}hehe"))

	log.Print(os.Expand("当前用户为：$USER", m))
	log.Print(os.Expand("我们使用$GOPROXY作为GO代理", m))
}
