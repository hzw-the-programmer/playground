git clone https://github.com/protocolbuffers/protobuf.git
git submodule update --init --recursive

/*
git clone https://github.com/chaconinc/MainProject
git submodule init
git submodule update

git clone --recurse-submodules https://github.com/chaconinc/MainProject
*/

git checkout v3.9.1
./autogen.sh
./configure --prefix=/home/zhiwenhe/build/protobuf
make
make check
make install

git clone https://github.com/golang/protobuf.git
cd protobuf
PATH=$PATH:/home/zhiwenhe/work/go/bin
make

PATH=$PATH:/home/zhiwenhe/go/bin
~/build/protobuf/bin/protoc --go_out=. addressbook.proto

mkdir -p ~/go/src/github.com/golang/protobuf
cd ~/go/src/github.com/golang/protobuf
cp -r ~/work/golang/protobuf/proto ./
cp -r ~/work/golang/protobuf/ptypes ./
