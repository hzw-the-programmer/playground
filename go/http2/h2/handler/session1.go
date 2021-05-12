package handler

import (
	"log"
	"net/http"
	"strings"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"
)

func Step1(c *gin.Context) {
	hzw1 := sessions.DefaultMany(c, "hzw1")
	hzw2 := sessions.DefaultMany(c, "hzw2")
	resHeader := c.Writer.Header()

	hzw1.Set("token", "token1234")
	hzw1.Save()

	var hzw1CL string
	for _, cl := range resHeader.Values("Set-Cookie") {
		i := strings.Index(cl, "hzw1=")
		j := strings.Index(cl, ";")
		if i != -1 && j != -1 {
			hzw1CL = cl[i:j]
			break
		}
	}

	resHeader.Del("Set-Cookie")

	hzw2.Set("hzw1CL", hzw1CL)
	hzw2.Save()

	var hzw2CV string
	for _, cl := range resHeader.Values("Set-Cookie") {
		i := strings.Index(cl, "hzw2=")
		j := strings.Index(cl, ";")
		if i != -1 && j != -1 {
			hzw2CV = cl[i+len("hzw2=") : j]
			break
		}
	}

	resHeader.Del("Set-Cookie")

	c.JSON(http.StatusOK, gin.H{
		"hzw1CL": hzw1CL,
		"hzw2CV": hzw2CV,
	})
}

func Step2(c *gin.Context) {
	cookies := c.Request.Cookies()
	c.Request.Header.Del("Cookie")
	for _, cookie := range cookies {
		if cookie.Name != "hzw1" && cookie.Name != "hzw2" {
			c.Request.AddCookie(cookie)
		}
	}

	hzw2 := sessions.DefaultMany(c, "hzw2")

	hzw2CV := c.Query("hzw2CV")
	c.Request.AddCookie(&http.Cookie{
		Name:  "hzw2",
		Value: hzw2CV,
	})
	hzw1CL, ok := hzw2.Get("hzw1CL").(string)
	hzw2.Set("hzw1CL", "")
	hzw2.Options(sessions.Options{
		MaxAge: -1,
	})
	hzw2.Save()

	log.Println(c.Writer.Header().Values("Set-Cookie"))
	c.Writer.Header().Del("Set-Cookie")

	if !ok {
		log.Println("hzw1CL is empty")
		c.JSON(http.StatusOK, "hzw1CL is empty")
		return
	}

	hzw1 := sessions.DefaultMany(c, "hzw1")

	c.Request.Header.Add("Cookie", hzw1CL)

	c.Writer.Header().Add("Set-Cookie", hzw1CL)

	c.JSON(http.StatusOK, gin.H{
		"token": hzw1.Get("token").(string),
	})
}
