package main

// mosquitto_pub -t "/topic0" -m "hello"

import (
	"flag"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/eclipse/paho.mqtt.golang"
)

func defaultHandler(c mqtt.Client, msg mqtt.Message) {
	log.Println("defaultHandler", msg.Topic(), string(msg.Payload()))
}

func topic0Handler(c mqtt.Client, msg mqtt.Message) {
	log.Println("topic0Handler", msg.Topic(), string(msg.Payload()))
}

func subscribeCleanSession() {
	o := mqtt.NewClientOptions()
	// o.SetCleanSession(false)
	// o.SetClientID("hzw")
	o.SetDefaultPublishHandler(defaultHandler)
	o.AddBroker(":1883")
	c := mqtt.NewClient(o)
	t := c.Connect()
	t.Wait()
	if t.Error() != nil {
		log.Fatal(t.Error())
	}
	c.AddRoute("/topic0", topic0Handler)
	if !t.(*mqtt.ConnectToken).SessionPresent() {
		log.Print("session not present")
		filters := map[string]byte{
			"/topic0": 0,
			"/topic1": 1,
			"/topic2": 2,
		}
		t = c.SubscribeMultiple(filters, nil)
		t.Wait()
		if t.Error() != nil {
			log.Fatal(t.Error())
		}
	} else {
		log.Print("session present")
	}
}

func subscribe() {
	o := mqtt.NewClientOptions()
	o.SetCleanSession(false)
	o.SetClientID("hzw")
	o.SetDefaultPublishHandler(defaultHandler)
	o.AddBroker(":1883")
	c := mqtt.NewClient(o)
	t := c.Connect()
	t.Wait()
	if t.Error() != nil {
		log.Fatal(t.Error())
	}
	c.AddRoute("/topic0", topic0Handler)
	if !t.(*mqtt.ConnectToken).SessionPresent() {
		log.Print("session not present")
		filters := map[string]byte{
			"/topic0": 0,
			"/topic1": 1,
			"/topic2": 2,
		}
		t = c.SubscribeMultiple(filters, nil)
		t.Wait()
		if t.Error() != nil {
			log.Fatal(t.Error())
		}
	} else {
		log.Print("session present")
	}
}

func publish() {

}

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)
	cmd := flag.Int("cmd", 1, "1: subscribeCleanSession; 2: subscribe; 3: publish")
	flag.Parse()
	if *cmd == 1 {
		subscribeCleanSession()
	} else if *cmd == 2 {
		subscribe()
	} else {
		publish()
	}

	sigChan := make(chan os.Signal)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	log.Print(<-sigChan)
}
