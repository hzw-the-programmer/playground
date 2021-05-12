mkdir grpc
cd grpc
vi greeter.proto #package main
PATH=$PATH:/home/zhiwenhe/build/protobuf/bin
PATH=$PATH:/home/zhiwenhe/go/bin
protoc --go_out=plugins=grpc:. greeter.proto

vi main.go
PATH=$PATH:/home/zhiwenhe/work/go/bin
go mod init grpc
export GOPROXY="https://goproxy.cn"
go build .

mkdir greeter
mv greeter.proto greeter
cd greeter
vi greeter.proto #package greeter
protoc --go_out=plugins=grpc:. greeter.proto
cd ..
mkdir server
mv main.go server
vi server/main.go #import grpc/greeter
go build -o s server/main.go

mkdir client
