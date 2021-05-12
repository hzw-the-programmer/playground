```shell
sudo vi ~/.bashrc
PATH="$HOME/bin/protobuf/bin:$PATH"
protoc --cpp_out=. addressbook.proto

sudo vi ~/.bashrc
PATH="$HOME/bin/protobuf/bin:$HOME/bin/protobuf-c/bin:$PATH"
protoc --c_out=. addressbook.proto

# only support protobuffers

#go get google.golang.org/protobuf/cmd/protoc-gen-go
##go get google.golang.org/protobuf/cmd/protoc-gen-go
##go install google.golang.org/protobuf/cmd/protoc-gen-go
#sudo vi ~/.bashrc
#PATH="$HOME/bin/protobuf/bin:$HOME/bin/protobuf-c/bin:$HOME/go/bin:$PATH"
#protoc --go_out=. addressbook.proto

# support protobuffers and grpc

#export GO111MODULE=on
go get github.com/golang/protobuf/protoc-gen-go
sudo vi ~/.bashrc
PATH="$HOME/bin/protobuf/bin:$HOME/bin/protobuf-c/bin:$HOME/go/bin:$PATH"
protoc --go_out=plugins=grpc:. greeter.proto

#go get google.golang.org/grpc
#protoc --go_out=plugins=grpc:. greeter.proto

cat addressbook.proto

syntax = "proto3";

import "google/protobuf/timestamp.proto";

message Person {
    string name = 1;
    int32 id = 2;
    string email = 3;

    enum PhoneType {
        MOBILE = 0;
        HOME = 1;
        WORK =2 ;
    }

    message PhoneNumber {
        string number = 1;
        PhoneType type = 2;
    }

    repeated PhoneNumber phones = 4;

    google.protobuf.Timestamp last_updated = 5;
}

message AddressBook {
    repeated Person people = 6;
}

```
