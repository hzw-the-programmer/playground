package main

import (
	"context"
	"log"
)

type EndPoint func(ctx context.Context, request interface{}) (response interface{}, err error)
type Middleware func(EndPoint) EndPoint

func m1(next EndPoint) EndPoint {
	return func(ctx context.Context, request interface{}) (response interface{}, err error) {
		log.Print("enter m1")
		response, err = next(ctx, request)
		log.Print("leave m1")
		return
	}
}

func m2(next EndPoint) EndPoint {
	return func(ctx context.Context, request interface{}) (response interface{}, err error) {
		log.Print("enter m2")
		return next(ctx, request)
	}
}

func m3(next EndPoint) EndPoint {
	return func(ctx context.Context, request interface{}) (response interface{}, err error) {
		log.Print("enter m3")
		return next(ctx, request)
	}
}

func chain(ms ...Middleware) Middleware {
	return func(next EndPoint) EndPoint {
		for i := len(ms) - 1; i >= 0; i-- {
			next = ms[i](next)
		}
		return next
	}
}

func h(ctx context.Context, request interface{}) (response interface{}, err error) {
	log.Printf("handler called, request: %v", request)
	return "ok", nil
}

func main() {
	response, _ := chain(m1, m2, m3)(h)(context.Background(), "hj")
	log.Printf("response: %v", response)
}
