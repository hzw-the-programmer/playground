package main

import (
	"os"

	"github.com/gin-contrib/sessions"
	"github.com/gin-contrib/sessions/cookie"
	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
	"github.com/mcuadros/go-gin-prometheus"

	"g1.hzw/handlers"
)

func main() {
	r := gin.Default()

	r.Use(static.ServeRoot("/static", "./static"))

	store := cookie.NewStore([]byte(os.Getenv("SESSION_KEY")))
	r.Use(sessions.Sessions("g1.hzw", store))

	p := ginprometheus.NewPrometheus("gin")
	p.Use(r)

	r.LoadHTMLGlob("templates/*")

	r.GET("/input_phone_number", handlers.InputPhoneNumber)
	r.POST("/send_verify_code", handlers.SendVerifyCode)
	r.GET("/input_verify_code", handlers.InputVerifyCode)

	r.RunTLS(":8080", "localhost.crt", "localhost.key")
}
