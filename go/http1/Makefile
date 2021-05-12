.PHONY: build

build:
	go fmt ./...
	# rice embed-go
	go build -o build/main
	GOOS=windows GOARCH=amd64 go build -o build/main.exe
