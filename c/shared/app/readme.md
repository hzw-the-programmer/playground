cd utils
gcc -c -Wall -Werror -fPIC foo.c
gcc -shared -o libfoo.so foo.o

cd ../main
gcc -Wall -o main main.c -v -I.. -lfoo -L../utils
