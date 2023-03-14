.PHONY: build

build:
	go fmt ./...
	go build -o build/main main.go
	GOOS=windows GOARCH=amd64 go build -o build/main.exe
	GOOS=linux GOARCH=amd64 go build -o build/main
