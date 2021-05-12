```
sudo apt update
sudo apt-get install autoconf automake libtool curl make g++ unzip pkg-config

git clone https://github.com/protocolbuffers/protobuf.git
cd protobuf
git submodule update --init --recursive
./autogen.sh

./configure
// ./configure --help
make
make check
sudo make install
sudo ldconfig
```

```
git clone https://github.com/protobuf-c/protobuf-c.git
cd protobuf-c/
./autogen.sh
./configure
make
sudo make install
```

```
protoc -Iproto --c_out=/mnt/c/work/tmp/proto/ proto/*.proto
```
