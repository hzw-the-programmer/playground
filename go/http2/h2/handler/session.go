package handler

import (
	"log"
	"net/http"
	"net/url"
	"strings"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"
)

func Two(c *gin.Context) {
	hzw := sessions.DefaultMany(c, "hzw")
	hzw1 := sessions.DefaultMany(c, "hzw1")
	hzw2 := sessions.DefaultMany(c, "hzw2")

	hzw.Set("hzw", "test")
	if err := hzw.Save(); err != nil {
		panic(err)
	}

	hzw1.Set("token", "tokenabcd")
	if err := hzw1.Save(); err != nil {
		panic(err)
	}

	var cv string
	for _, v := range c.Writer.Header().Values("Set-Cookie") {
		log.Println(v)
		if strings.HasPrefix(v, "hzw1") {
			cv = v
		}
	}

	hzw2.Set("cv", cv)
	if err := hzw2.Save(); err != nil {
		panic(err)
	}

	for _, v := range c.Writer.Header().Values("Set-Cookie") {
		log.Println(v)
		log.Println(url.QueryEscape(v))
	}

	c.Writer.Header().Del("Set-Cookie")

	c.JSON(http.StatusOK, "two")
}

func One(c *gin.Context) {
	cookie := c.Query("hzw2")
	// log.Println(cookie)
	// for _, c := range c.Request.Cookies() {
	// 	log.Println(c.Name, c.Value)
	// }

	// c.Request.Header["Cookie"] = []string{cookie}
	c.Request.AddCookie(&http.Cookie{
		Name:  "hzw2",
		Value: cookie,
	})
	hzw2 := sessions.DefaultMany(c, "hzw2")
	cv := hzw2.Get("cv").(string)
	log.Println("cv", cv)

	c.Request.Header["Cookie"] = []string{cv}
	hzw1 := sessions.DefaultMany(c, "hzw1")
	log.Println(hzw1.Get("token"))

	c.JSON(http.StatusOK, "one")
}

func St1(c *gin.Context) {
	log.Println("cookies:")
	for _, c := range c.Request.Header["Cookie"] {
		log.Println(c)
	}
	c.Request.AddCookie(&http.Cookie{
		Name:  "hzw",
		Value: "haha",
	})
	c.Request.AddCookie(&http.Cookie{
		Name:  "hzw",
		Value: "henhen",
	})
	log.Println("cookies:")
	for _, c := range c.Request.Header["Cookie"] {
		log.Println(c)
	}

	hzw, _ := c.Request.Cookie("hzw")
	log.Println(hzw.Value)

	c.JSON(http.StatusOK, "st1")
}

func St2(c *gin.Context) {
	c.Request.AddCookie(&http.Cookie{
		Name:  "c1",
		Value: "a",
	})
	c.Request.AddCookie(&http.Cookie{
		Name:  "c2",
		Value: "b",
	})
	c.Request.AddCookie(&http.Cookie{
		Name:  "c1",
		Value: "c",
	})
	c.Request.Header.Add("Cookie", "c1=d")
	c.Request.Header.Add("Cookie", "c2=e")
	c.Request.Header.Add("Cookie", "c1=f")
	c.Request.Header.Add("Cookie", "c1=g")
	// c.Request.AddCookie(&http.Cookie{
	// 	Name:  "c3",
	// 	Value: "h",
	// })
	for _, c := range c.Request.Header["Cookie"] {
		log.Println(c)
	}
	log.Println()
	cookies := c.Request.Cookies()
	for _, c := range cookies {
		log.Println(c.Name, c.Value)
	}
	log.Println()
	c.Request.Header.Del("Cookie")
	log.Println(len(c.Request.Cookies()))
	var i int
	for _, c := range cookies {
		if c.Name != "c1" {
			cookies[i] = c
			i++
		}
	}
	cookies = cookies[:i]
	for _, c := range cookies {
		log.Println(c.Name, c.Value)
	}
	log.Println()

	for _, c1 := range cookies {
		c.Request.AddCookie(c1)
	}

	log.Println("after add cookie")

	cookies = c.Request.Cookies()
	for _, c := range cookies {
		log.Println(c.Name, c.Value)
	}
	log.Println()

	c.JSON(http.StatusOK, "st2")
}
