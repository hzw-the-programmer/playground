package main

import (
	"log"
	"math"
)

type Handler func(*Context)

type Context struct {
	handlers []Handler
	index    int8
}

const abortIndex = math.MaxInt8 / 2

func NewContext() *Context {
	return &Context{index: -1}
}

func (ctx *Context) AddHandlers(handlers ...Handler) {
	ctx.handlers = append(ctx.handlers, handlers...)
}

func (ctx *Context) Next() {
	ctx.index++
	for ctx.index < int8(len(ctx.handlers)) {
		ctx.handlers[ctx.index](ctx)
		ctx.index++
	}
}

func (ctx *Context) Abort() {
	ctx.index = abortIndex
}

func (ctx *Context) IsAborted() bool {
	return ctx.index >= abortIndex
}

func bitStr(n int8) string {
	str := ""
	for i := 7; i >= 0; i-- {
		if n&(1<<i) != 0 {
			str += "1"
		} else {
			str += "0"
		}
	}
	return str
}

func main() {
	log.Print(math.MaxInt8, math.MaxInt8/2)
	var n int8
	for {
		log.Println(n, bitStr(n))
		n += 1
		if n == 0 {
			log.Println(n, bitStr(n))
			break
		}
	}

	ctx := NewContext()

	for i := 0; i < math.MaxInt8/2; i++ {
		// for i := 0; i < math.MaxInt8; i++ {
		j := i
		ctx.AddHandlers(func(ctx *Context) {
			log.Printf("enter %d", j)
			ctx.Next()
			log.Print(ctx.index)
			log.Printf("leave %d", j)
		})
	}

	ctx.Next()
	log.Print(ctx.index)
	log.Print(ctx.IsAborted())
}
