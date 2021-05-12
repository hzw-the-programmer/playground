.PHONY: build
VERSION := $(shell git describe --always)

build:
	go fmt ./...
	go build -ldflags "-X main.version=$(VERSION)" -o build/lorawanapp main.go

test:
	go fmt ./...
	go test ./...
