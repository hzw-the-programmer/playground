package main

type hub struct {
	clients    map[*client]bool
	register   chan *client
	unregister chan *client
	broadcast  chan string
}

func NewHub() *hub {
	return &hub{
		clients:    make(map[*client]bool),
		register:   make(chan *client),
		unregister: make(chan *client),
		broadcast:  make(chan string),
	}
}

func (h *hub) Register(c *client) {
	h.register <- c
	c.SetHub(h)
}

func (h *hub) Unregister(c *client) {
	h.unregister <- c
}

func (h *hub) Run() {
	for {
		select {
		case c := <-h.register:
			h.clients[c] = true
		case c := <-h.unregister:
			delete(h.clients, c)
		case msg := <-h.broadcast:
			for c := range h.clients {
				c.Writech() <- msg
			}
		}
	}
}

func (h *hub) Broadcast() chan string {
	return h.broadcast
}
