nm mod1.o | grep _GLOBAL_OFFSET_TABLE_
readelf -s mod1.o | grep _GLOBAL_OFFSET_TABLE_

objdump --all-headers libmod.so | grep TEXTREL
readelf -s libmod.so | grep TEXTREL

objdump -p libhzw.so.1.0.1 | grep SONAME
readelf -d libhzw.so.1.0.1 | grep SONAME

ldconfig -Nv
ldconfig -Xv
ldconfig -v
ldconfig -nv .

LD_LIBRARY_PATH=. ldd ./prog
