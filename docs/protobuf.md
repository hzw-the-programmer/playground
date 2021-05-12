git clone https://github.com/protocolbuffers/protobuf.git
git submodule update --init --recursive
git checkout v3.9.1
./autogen.sh
./configure --prefix=/home/zhiwenhe/build/protobuf
make
make check
make install
