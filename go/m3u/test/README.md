mkdir m3u
cd m3u
cobra init test --pkg-name github.com/hzw-the-programmer/gopkg/test
cd test
go mod init github.com/hzw-the-programmer/gopkg/test
go mod tidy
go run .
go mod edit --replace example.com/greetings=../../module/greetings
go mod tidy
