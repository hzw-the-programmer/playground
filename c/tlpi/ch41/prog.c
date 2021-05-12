// static library

// cc -g -c mod1.c mod2.c
// ar r libmod.a mod1.o mod2.o
// cc -g -c prog.c

// cc -g -o prog prog.o libmod.a
// or
// cc -g -o prog prog.o -lmod -L.

// shared library

// gcc -g -c -fPIC -Wall mod1.c mod2.c

// gcc -g --shared -o libmod.so mod1.o mod2.o
// gcc -g -Wall -o prog prog.c libmod.so
// LD_LIBRARY_PATH=. ./prog

// or

// gcc -g --shared -Wl,-soname,libmodsn.so -o libmod.so mod1.o mod2.o
// gcc -g -Wall -o prog prog.c libmod.so
// ln -s libmod.so libmodsn.so
// LD_LIBRARY_PATH=. ./prog

// objdump -p libmod.so | grep SONAME
// readelf -d libmod.so | grep SONAME

// ******
// gcc -g -c -fPIC -Wall mod1.c mod2.c
// gcc -g --shared -Wl,-soname,libhzw.so.1 -o libhzw.so.1.0.1 mod1.o mod2.o
// ln -s libhzw.so.1.0.1 libhzw.so.1
// ln -s libhzw.so.1 libhzw.so
// gcc -g -Wall -o prog prog.c -L. -lhzw
// LD_LIBRARY_PATH=. ./prog

// gcc -g -c -fPIC -Wall mod1.c mod2.c
// gcc -g --shared -Wl,-soname,libhzw.so.1 -o libhzw.so.1.0.2 mod1.o mod2.o
// ln -fs libhzw.so.1.0.2 libhzw.so.1
// ******

// ******
// gcc -g -c -fPIC -Wall mod1.c mod2.c
// gcc -g --shared -Wl,-soname,libhzw.so.1 -o libhzw.so.1.0.1 mod1.o mod2.o
// edit
// gcc -g -c -fPIC -Wall mod1.c mod2.c
// gcc -g --shared -Wl,-soname,libhzw.so.1 -o libhzw.so.1.0.2 mod1.o mod2.o
// edit
// gcc -g -c -fPIC -Wall mod1.c mod2.c
// gcc -g --shared -Wl,-soname,libhzw.so.2 -o libhzw.so.2.0.1 mod1.o mod2.o
// edit
// gcc -g -c -fPIC -Wall mod1.c mod2.c
// gcc -g --shared -Wl,-soname,libhzw.so.2 -o libhzw.so.2.0.2 mod1.o mod2.o
// ldconfig -nv .
// ln -s libhzw.so.1 libhzw.so
// gcc -g -Wall -o prog prog.c -L. -lhzw
// LD_LIBRARY_PATH=. ./prog
// gcc -g --shared -Wl,-soname,libhzw.so.2 -o libhzw.so.1.0.2 mod1.o mod2.o
// gcc -g -Wall -o prog prog.c -L. -lhzw
// LD_LIBRARY_PATH=. ./prog
// ******

void mod1_func();
void mod2_func();

int main(int argc, char* argv[]) {
    mod1_func();
    mod2_func();
    return 0;
}
