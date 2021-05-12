package main

import (
	"fmt"
	"math/rand"
	"net/http"
	"io"

	"github.com/gin-gonic/gin"
)

var roomManager *RoomManager

func main() {
	roomManager = NewRoomManager()
	
	r := gin.Default()
	r.LoadHTMLGlob("./templates/*")

	r.Static("/public", "./public")

	r.GET("/room/:roomid", roomGet)
	r.POST("/room/:roomid", roomPost)

	r.GET("/stream/:roomid", stream)

	r.Run()
}

func roomGet(c *gin.Context) {
	roomid := c.Param("roomid")
	userid := fmt.Sprint(rand.Int31())
	
	c.HTML(http.StatusOK, "chat_room.html", gin.H{
		"roomid": roomid,
		"userid": userid,
	})
}

func roomPost(c *gin.Context) {
	roomid := c.Param("roomid")
	userid := c.PostForm("userid")
	message := c.PostForm("message")

	roomManager.Submit(roomid, userid, message)

	// time.Sleep(3 * time.Second)

	c.JSON(http.StatusOK, gin.H{
		"roomid": roomid,
		"userid": userid,
		"message": message,
		"status": "success",
	})
}

func stream(c *gin.Context) {
	roomid := c.Param("roomid")

	channel := roomManager.OpenListener(roomid)
	defer roomManager.CloseListener(roomid, channel)

	clientGone := c.Writer.CloseNotify()

	c.Stream(func (w io.Writer) bool {
		select {
		case <-clientGone:
			return false;
		case message := <-channel:
			c.SSEvent("message", message)
			return true
		}
	})
}
