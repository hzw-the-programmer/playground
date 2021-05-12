package main

import (
	// "crypto/rand"
	// "io"
	"log"
	"net/http"
	"net/http/httptest"
	"os"

	"github.com/boj/redistore"
	"github.com/gorilla/sessions"
)

var (
	authKey = []byte{0x2, 0x8c, 0xde, 0xa0, 0x20, 0x6, 0x9f, 0xbf, 0x26, 0x1a, 0xc8, 0x5a, 0x52, 0x0, 0xa1, 0x4e, 0xb9, 0x63, 0xe0, 0xc, 0x5, 0xa5, 0x8f, 0x42, 0x9e, 0xfe, 0x5c, 0x5e, 0x23, 0x63, 0x28, 0x98}
	encKey  = []byte{0x4, 0xd3, 0xa9, 0xc3, 0x12, 0x24, 0x8c, 0xf1, 0xeb, 0x66, 0x93, 0xfb, 0xaa, 0x67, 0xbf, 0x19, 0xb9, 0xc4, 0x58, 0x53, 0x54, 0xdb, 0xb, 0xfd, 0xff, 0xee, 0xca, 0x86, 0x62, 0x2b, 0xb7, 0x4c}
)

func main() {
	addr := os.Args[1]
	var store sessions.Store

	testStore(sessions.NewCookieStore(authKey, encKey))
	testStore(sessions.NewFilesystemStore("./", authKey, encKey))

	store, err := redistore.NewRediStore(3, "tcp", addr, "", authKey, encKey)
	if err != nil {
		log.Print(err)
	}
	testStore(store)
}

func testStore(store sessions.Store) {
	log.Printf("%#v", store)
	r, _ := http.NewRequest(http.MethodGet, "www.example.com", nil)

	// authKey := make([]byte, 32)
	// encKey := make([]byte, 32)
	// io.ReadFull(rand.Reader, authKey)
	// io.ReadFull(rand.Reader, encKey)
	// log.Printf("%#v", authKey)
	// log.Printf("%#v", encKey)
	// store := sessions.NewCookieStore(authKey, encKey)

	session, _ := store.Get(r, "hzw")
	session1, _ := store.Get(r, "hzw")
	if session != session1 {
		log.Print("session != session1")
	}

	w := httptest.NewRecorder()

	session.Values["count"] = 1
	session.Save(r, w)
	log.Print(w.Header().Values("Set-Cookie"))

	r.Header["Cookie"] = w.Header().Values("Set-Cookie")
	session2, _ := store.Get(r, "hzw")
	if session != session2 {
		log.Print("session != session2")
	}
	log.Printf("%+v", session2)

	session3, _ := store.New(r, "hzw")
	if session == session3 {
		log.Print("session == session3")
	}
	log.Printf("%+v", session3)
}
