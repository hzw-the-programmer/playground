Makefile_sprd
```
SOURCES = aes.o

testaes: $(SOURCES) testaes.o
```

c:\code\cifra\src>..\..\tools\make.exe -f Makefile_sprd
cc    -c -o testaes.o testaes.c

Makefile_sprd
```
CFLAGS += -g

SOURCES = aes.o

testaes: $(SOURCES) testaes.o
```

c:\code\cifra\src>..\..\tools\make.exe -f Makefile_sprd
cc -g   -c -o testaes.o testaes.c

Makefile_sprd
```
CC = tcc
CPPFLAGS += -I./ext

SOURCES = aes.o

testaes: $(SOURCES) testaes.o
```

c:\code\cifra\src>..\..\tools\make.exe -f Makefile_sprd
tcc  -I./ext  -c -o testaes.o testaes.c