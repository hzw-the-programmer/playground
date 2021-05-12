package main

import (
	"log"
	"os"
	"time"

	"gopkg.in/yaml.v2"
)

type redisConfig struct {
	Addr    string        `yaml:"addr"`
	Timeout time.Duration `yaml:"timeout"`
	// Timeout1 int64         `yaml:"timeout1"`
	addr1 string `yaml:"addr1"`
	addr2 string
	Addr3 string
	Addr4 string `yaml:"Addr4"`
}

type config struct {
	Redis redisConfig `yaml:"redis"`
}

func main() {
	var c config
	f, err := os.Open("./base.yaml")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	decoder := yaml.NewDecoder(f)
	err = decoder.Decode(&c)
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("%+v", c)
	log.Print(int64(c.Redis.Timeout))
}
