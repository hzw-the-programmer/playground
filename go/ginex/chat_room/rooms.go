package main

import(
	"github.com/dustin/go-broadcast"
)

type RoomManager struct {
	open chan *Listener
	close chan *Listener
	rooms map[string]broadcast.Broadcaster
	messages chan *Message
}

type Listener struct {
	RoomId string
	Channel chan interface{}
}

type Message struct {
	RoomId string
	UserId string
	Text string
}

func NewRoomManager() *RoomManager {
	rm := &RoomManager{
		open: make(chan *Listener, 100),
		close: make(chan *Listener, 100),
		rooms: make(map[string]broadcast.Broadcaster),
		messages: make(chan *Message, 100),
	}

	go rm.run()

	return rm;
}

func (rm *RoomManager) OpenListener(roomid string) chan interface{} {
	channel := make(chan interface{})

	rm.open <- &Listener{
		RoomId: roomid,
		Channel: channel,
	}

	return channel
}

func (rm *RoomManager) CloseListener(roomid string, channel chan interface{}) {
	rm.close <- &Listener{
		RoomId: roomid,
		Channel: channel,
	}
}

func (rm *RoomManager) Submit(roomid, userid, text string) {
	rm.messages <- &Message{
		RoomId: roomid,
		UserId: userid,
		Text: text,
	}
}

func (rm *RoomManager) run() {
	for {
		select {
		case listener := <-rm.open:
			rm.register(listener)
		case listener := <-rm.close:
			rm.unregister(listener)
		case message := <- rm.messages:
			rm.room(message.RoomId).Submit(message.UserId + ": " + message.Text)
		}
	}
}

func (rm *RoomManager) register(listener *Listener) {
	rm.room(listener.RoomId).Register(listener.Channel)
}

func (rm *RoomManager) unregister(listener *Listener) {
	rm.room(listener.RoomId).Unregister(listener.Channel)
	close(listener.Channel)
}

func (rm *RoomManager) room(roomid string) broadcast.Broadcaster {
	r, ok := rm.rooms[roomid]
	if !ok {
		r = broadcast.NewBroadcaster(10)
		rm.rooms[roomid] = r
	}
	return r
}
