mkdir greetings
cd greetings
go mod init example.com/greetings
vi greetings.go

cd ..
mkdir hello
cd hello
go mod init example.com/hello
vi main.go
go mod edit --replace example.com/greetings=../greetings
go mod tidy
go run .

go fmt
go test
go test -v
