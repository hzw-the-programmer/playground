git clone https://github.com/protocolbuffers/protobuf.git
git submodule update --init --recursive
git checkout v3.9.1
./autogen.sh
./configure --prefix=/home/ml/bin/protobuf
make
make check
make install

# cmake

git clone git@github.com:hzw-the-programmer/protobuf.git
cd protobuf
git submodule update --init --recursive
mkdir build
cd build
cmake ..
cmake --build .
cmake --install-prefix /home/ml/bin/protobuf .
cmake --install .
