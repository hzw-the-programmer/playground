cd build/
tar -xvf ../zips/tcl8.7a1-src.tar.gz
cd tcl8.7a1/unix/
./configure --prefix=/home/pageask/build/tcl8.7a1-build
make
make install

cd ../../
tar -xvf ../zips/tk8.7a1-src.tar.gz
cd tk8.7a1/unix/
./configure --with-tcl=/home/pageask/build/tcl8.7a1/unix --prefix=/home/pageask/build/tk8.7a1-build
make
make install

cd ../../Python-3.6.2
./configure --help
./configure --with-tcltk-includes='-I /home/pageask/build/tcl8.7a1-build/include -I /home/pageask/build/tk8.7a1-build/include' --with-tcltk-libs='-L/home/pageask/build/tcl8.7a1-build/lib -L/home/pageask/build/tk8.7a1-build/lib' --prefix=/home/pageask/build/Python-3.6.2-build
make

cd ..
tar -xvf ../zips/bzip2-1.0.6.tar.gz
cd bzip2-1.0.6/
make
make install PREFIX=/home/pageask/build/bzip2-1.0.6-build

cd ../Python-3.6.2
./configure --with-tcltk-includes='-I /home/pageask/build/tcl8.7a1-build/include -I /home/pageask/build/tk8.7a1-build/include' --with-tcltk-libs='-L/home/pageask/build/tcl8.7a1-build/lib -L/home/pageask/build/tk8.7a1-build/lib' --prefix=/home/pageask/build/Python-3.6.2-build LDFLAGS=-L/home/pageask/build/bzip2-1.0.6-build/lib CPPFLAGS=-I/home/pageask/build/bzip2-1.0.6-build/include
make

cd ../bzip2-1.0.6/
make clean
make CFLAGS=-fPIC
make install PREFIX=/home/pageask/build/bzip2-1.0.6-build

cd ../Python-3.6.2
make

cd ..
tar -xvf ../zips/gdbm-1.13.tar.gz
cd gdbm-1.13/
<!-- ./configure --prefix=/home/pageask/build/gdbm-1.13-build -->
./configure --enable-libgdbm-compat --prefix=/home/pageask/build/gdbm-1.13-build
make
make install

cd ../Python-3.6.2
./configure --with-tcltk-includes='-I/home/pageask/build/tcl8.7a1-build/include -I/home/pageask/build/tk8.7a1-build/include' --with-tcltk-libs='-L/home/pageask/build/tcl8.7a1-build/lib -L/home/pageask/build/tk8.7a1-build/lib' --prefix=/home/pageask/build/Python-3.6.2-build LDFLAGS='-L/home/pageask/build/bzip2-1.0.6-build/lib -L/home/pageask/build/gdbm-1.13-build/lib' CPPFLAGS='-I/home/pageask/build/bzip2-1.0.6-build/include -I/home/pageask/build/gdbm-1.13-build/include'
LD_LIBRARY_PATH=/home/pageask/build/gdbm-1.13-build/lib make

./configure --prefix=/home/pageask/build/Python-3.6.2-build LDFLAGS='-L/home/pageask/build/bzip2-1.0.6-build/lib -L/home/pageask/build/gdbm-1.13-build/lib -L/home/pageask/build/tcl8.7a1-build/lib -L/home/pageask/build/tk8.7a1-build/lib' CPPFLAGS='-I/home/pageask/build/bzip2-1.0.6-build/include -I/home/pageask/build/gdbm-1.13-build/include -I/home/pageask/build/tcl8.7a1-build/include -I/home/pageask/build/tk8.7a1-build/include'
LD_LIBRARY_PATH=/home/pageask/build/gdbm-1.13-build/lib make
make install

cd ..
tar -xvf ../zips/tcl8.6.7-src.tar.gz
cd tcl8.6.7/unix/
./configure --prefix=/home/pageask/build/tcl8.6.7-build
make
make install

cd ../..
tar -xvf ../zips/tk8.6.7-src.tar.gz
cd tk8.6.7/unix/
./configure --prefix=/home/pageask/build/tk8.6.7-build --with-tcl=/home/pageask/build/tcl8.6.7/unix
make
make install

cd ../../Python-3.6.2
./configure --prefix=/home/pageask/build/Python-3.6.2-build LDFLAGS='-L/home/pageask/build/bzip2-1.0.6-build/lib -L/home/pageask/build/gdbm-1.13-build/lib -L/home/pageask/build/tcl8.6.7-build/lib -L/home/pageask/build/tk8.6.7-build/lib' CPPFLAGS='-I/home/pageask/build/bzip2-1.0.6-build/include -I/home/pageask/build/gdbm-1.13-build/include -I/home/pageask/build/tcl8.6.7-build/include -I/home/pageask/build/tk8.6.7-build/include'
make clean
LD_LIBRARY_PATH=/home/pageask/build/gdbm-1.13-build/lib make
make install
