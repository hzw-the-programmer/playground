build:
	go fmt ./...
	go build main.go array.go
	GOOS=windows GOARCH=amd64 go build main.go array.go

clean:
	rm main main.exe
