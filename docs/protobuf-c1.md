PKG_CONFIG_PATH=~/build/protobuf/lib/pkgconfig/ pkg-config --cflags protobuf
PKG_CONFIG_PATH=~/build/protobuf/lib/pkgconfig/ pkg-config --libs protobuf
PKG_CONFIG_PATH=~/build/protobuf/lib/pkgconfig/ pkg-config --cflags --libs protobuf

./autogen.sh
PKG_CONFIG_PATH=~/build/protobuf/lib/pkgconfig/ ./configure --prefix=/home/zhiwenhe/build/protobuf-c
make
make install

~/build/protobuf-c/bin/protoc-c --c_out=. system.proto people.proto
