complier: https://github.com/protocolbuffers/protobuf
release: https://github.com/protocolbuffers/protobuf/releases

plugin & runtime: https://github.com/protocolbuffers/protobuf-go.git

open git cmd line
echo $PATH
C:\Program Files\Git\mingw64\bin
C:\Program Files\Git\usr\bin
C:\Go\bin

mkdir C:\Users\Admin\bin
copy protoc.exe to C:\Users\Admin\bin

go env
GOBIN=
GOPATH=C:\Users\Admin\go

go install google.golang.org/protobuf/cmd/protoc-gen-go
ls C:\Users\Admin\go\bin

protoc.exe --go_out=. addressbook.proto
