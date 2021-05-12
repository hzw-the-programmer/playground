```shell
git clone https://github.com/protobuf-c/protobuf-c.git
cd protobuf-c/
./autogen.sh
PKG_CONFIG_PATH=/home/hezhiwen/bin/protobuf/lib/pkgconfig/ ./configure --prefix=/home/hezhiwen/bin/protobuf-c
make
make install
```

# output of ./autogen.sh

autoreconf: Entering directory `.'
autoreconf: configure.ac: not using Gettext
autoreconf: running: aclocal --force -I m4 ${ACLOCAL_FLAGS}
autoreconf: configure.ac: tracing
autoreconf: configure.ac: creating directory build-aux
autoreconf: running: libtoolize --copy --force
libtoolize: putting auxiliary files in AC_CONFIG_AUX_DIR, 'build-aux'.
libtoolize: copying file 'build-aux/ltmain.sh'
libtoolize: putting macros in AC_CONFIG_MACRO_DIRS, 'm4'.
libtoolize: copying file 'm4/libtool.m4'
libtoolize: copying file 'm4/ltoptions.m4'
libtoolize: copying file 'm4/ltsugar.m4'
libtoolize: copying file 'm4/ltversion.m4'
libtoolize: copying file 'm4/lt~obsolete.m4'
autoreconf: running: /usr/bin/autoconf --force
autoreconf: running: /usr/bin/autoheader --force
autoreconf: running: automake --add-missing --copy --force-missing
configure.ac:14: installing 'build-aux/compile'
configure.ac:22: installing 'build-aux/config.guess'
configure.ac:22: installing 'build-aux/config.sub'
configure.ac:13: installing 'build-aux/install-sh'
configure.ac:13: installing 'build-aux/missing'
Makefile.am: installing 'build-aux/depcomp'
parallel-tests: installing 'build-aux/test-driver'
autoreconf: Leaving directory `.'

# output of ./configure

checking for a BSD-compatible install... /usr/bin/install -c
checking whether build environment is sane... yes
checking for a thread-safe mkdir -p... /bin/mkdir -p
checking for gawk... no
checking for mawk... mawk
checking whether make sets $(MAKE)... yes
checking whether make supports nested variables... yes
checking for style of include used by make... GNU
checking for gcc... gcc
checking whether the C compiler works... yes
checking for C compiler default output file name... a.out
checking for suffix of executables... 
checking whether we are cross compiling... no
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc accepts -g... yes
checking for gcc option to accept ISO C89... none needed
checking whether gcc understands -c and -o together... yes
checking dependency style of gcc... gcc3
checking for gcc option to accept ISO C99... none needed
checking for gcc option to accept ISO Standard C... (cached) none needed
checking for g++... g++
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking dependency style of g++... gcc3
checking whether ln -s works... yes
checking how to run the C preprocessor... gcc -E
checking for grep that handles long lines and -e... /bin/grep
checking for egrep... /bin/grep -E
checking for ANSI C header files... yes
checking for sys/types.h... yes
checking for sys/stat.h... yes
checking for stdlib.h... yes
checking for string.h... yes
checking for memory.h... yes
checking for strings.h... yes
checking for inttypes.h... yes
checking for stdint.h... yes
checking for unistd.h... yes
checking minix/config.h usability... no
checking minix/config.h presence... no
checking for minix/config.h... no
checking whether it is safe to define __EXTENSIONS__... yes
checking for special C compiler options needed for large files... no
checking for _FILE_OFFSET_BITS value needed for large files... no
checking whether make supports nested variables... (cached) yes
checking build system type... x86_64-pc-linux-gnu
checking host system type... x86_64-pc-linux-gnu
checking how to print strings... printf
checking for a sed that does not truncate output... /bin/sed
checking for fgrep... /bin/grep -F
checking for ld used by gcc... /usr/bin/ld
checking if the linker (/usr/bin/ld) is GNU ld... yes
checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
checking the name lister (/usr/bin/nm -B) interface... BSD nm
checking the maximum length of command line arguments... 1572864
checking how to convert x86_64-pc-linux-gnu file names to x86_64-pc-linux-gnu format... func_convert_file_noop
checking how to convert x86_64-pc-linux-gnu file names to toolchain format... func_convert_file_noop
checking for /usr/bin/ld option to reload object files... -r
checking for objdump... objdump
checking how to recognize dependent libraries... pass_all
checking for dlltool... no
checking how to associate runtime and link libraries... printf %s\n
checking for ar... ar
checking for archiver @FILE support... @
checking for strip... strip
checking for ranlib... ranlib
checking command to parse /usr/bin/nm -B output from gcc object... ok
checking for sysroot... no
checking for a working dd... /bin/dd
checking how to truncate binary pipes... /bin/dd bs=4096 count=1
checking for mt... mt
checking if mt is a manifest tool... no
checking for dlfcn.h... yes
checking for objdir... .libs
checking if gcc supports -fno-rtti -fno-exceptions... no
checking for gcc option to produce PIC... -fPIC -DPIC
checking if gcc PIC flag -fPIC -DPIC works... yes
checking if gcc static flag -static works... yes
checking if gcc supports -c -o file.o... yes
checking if gcc supports -c -o file.o... (cached) yes
checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
checking whether -lc should be explicitly linked in... no
checking dynamic linker characteristics... GNU/Linux ld.so
checking how to hardcode library paths into programs... immediate
checking whether stripping libraries is possible... yes
checking if libtool supports shared libraries... yes
checking whether to build shared libraries... yes
checking whether to build static libraries... yes
checking how to run the C++ preprocessor... g++ -E
checking for ld used by g++... /usr/bin/ld -m elf_x86_64
checking if the linker (/usr/bin/ld -m elf_x86_64) is GNU ld... yes
checking whether the g++ linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
checking for g++ option to produce PIC... -fPIC -DPIC
checking if g++ PIC flag -fPIC -DPIC works... yes
checking if g++ static flag -static works... yes
checking if g++ supports -c -o file.o... yes
checking if g++ supports -c -o file.o... (cached) yes
checking whether the g++ linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
checking dynamic linker characteristics... (cached) GNU/Linux ld.so
checking how to hardcode library paths into programs... immediate
checking whether C compiler accepts "-Wc99-c11-compat"... yes
checking whether C compiler accepts "-Werror=incompatible-pointer-types"... yes
checking whether C compiler accepts "-Werror=int-conversion"... yes
checking whether C compiler accepts "-Wnull-dereference"... yes
checking for doxygen... no
checking for pkg-config... /usr/bin/pkg-config
checking pkg-config is at least version 0.9.0... yes
checking whether g++ supports C++11 features with -std=c++11... yes
checking for protobuf... yes
checking google/protobuf/compiler/command_line_interface.h usability... yes
checking google/protobuf/compiler/command_line_interface.h presence... yes
checking for google/protobuf/compiler/command_line_interface.h... yes
checking for protoc... /home/hezhiwen/bin/protobuf/bin/protoc
checking if LD -Wl,--version-script works... yes
checking whether self tests are run under valgrind... no
checking whether to build with code coverage support... no
checking whether byte ordering is bigendian... no
checking that generated files are newer than configure... done
configure: creating ./config.status
config.status: creating Makefile
config.status: creating protobuf-c/libprotobuf-c.pc
config.status: creating config.h
config.status: executing depfiles commands
config.status: executing libtool commands

    protobuf-c 1.3.3

        CC:                     gcc
        CFLAGS:                 -g -O2
        CXX:                    g++ -std=c++11
        CXXFLAGS:               -g -O2
        LDFLAGS:                
        LIBS:                   

        prefix:                 /home/hezhiwen/bin/protobuf-c
        sysconfdir:             ${prefix}/etc
        libdir:                 ${exec_prefix}/lib
        includedir:             ${prefix}/include
        pkgconfigdir:           ${libdir}/pkgconfig

        bigendian:              no
        protobuf version:       libprotoc 3.11.4

# output of make

  CXX      protoc-c/protoc_c_protoc_gen_c-c_bytes_field.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_enum.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_enum_field.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_extension.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_field.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_file.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_generator.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_helpers.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_message.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_message_field.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_primitive_field.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_service.o
  CXX      protoc-c/protoc_c_protoc_gen_c-c_string_field.o
  CXX      protoc-c/protoc_c_protoc_gen_c-main.o
  CXXLD    protoc-c/protoc-gen-c
  GEN      t/test.pb-c.c
[libprotobuf WARNING google/protobuf/compiler/parser.cc:648] No syntax specified for the proto file: t/test.proto. Please use 'syntax = "proto2";' or 'syntax = "proto3";' to specify a syntax version. (Defaulted to proto2 syntax.)
  GEN      t/test-full.pb-c.c
[libprotobuf WARNING google/protobuf/compiler/parser.cc:648] No syntax specified for the proto file: t/test-full.proto. Please use 'syntax = "proto2";' or 'syntax = "proto3";' to specify a syntax version. (Defaulted to proto2 syntax.)
  GEN      t/test-optimized.pb-c.c
[libprotobuf WARNING google/protobuf/compiler/parser.cc:648] No syntax specified for the proto file: t/test-optimized.proto. Please use 'syntax = "proto2";' or 'syntax = "proto3";' to specify a syntax version. (Defaulted to proto2 syntax.)
  GEN      t/test-full.pb.cc
[libprotobuf WARNING google/protobuf/compiler/parser.cc:648] No syntax specified for the proto file: t/test-full.proto. Please use 'syntax = "proto2";' or 'syntax = "proto3";' to specify a syntax version. (Defaulted to proto2 syntax.)
  CXX      t/generated-code2/t_generated_code2_cxx_generate_packed_data-cxx-generate-packed-data.o
  CXX      t/t_generated_code2_cxx_generate_packed_data-test-full.pb.o
  CXXLD    t/generated-code2/cxx-generate-packed-data
  GEN      t/generated-code2/test-full-cxx-output.inc
  GEN      t/test-proto3.pb-c.c
  GEN      t/issue220/issue220.pb-c.c
[libprotobuf WARNING google/protobuf/compiler/parser.cc:648] No syntax specified for the proto file: t/issue220/issue220.proto. Please use 'syntax = "proto2";' or 'syntax = "proto3";' to specify a syntax version. (Defaulted to proto2 syntax.)
  GEN      t/issue251/issue251.pb-c.c
[libprotobuf WARNING google/protobuf/compiler/parser.cc:648] No syntax specified for the proto file: t/issue251/issue251.proto. Please use 'syntax = "proto2";' or 'syntax = "proto3";' to specify a syntax version. (Defaulted to proto2 syntax.)
  GEN      t/issue330/issue330.pb-c.c
  GEN      t/issue375/issue375.pb-c.c
make  all-am
make[1]: 进入目录“/home/hezhiwen/work/protobuf-c”
  CC       protobuf-c/protobuf-c.lo
  CCLD     protobuf-c/libprotobuf-c.la
ar: `u' modifier ignored since `D' is the default (see `U')
make[1]: 离开目录“/home/hezhiwen/work/protobuf-c”

# output of make install

make  install-am
make[1]: 进入目录“/home/hezhiwen/work/protobuf-c”
make[2]: 进入目录“/home/hezhiwen/work/protobuf-c”
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf-c/lib'
 /bin/bash ./libtool   --mode=install /usr/bin/install -c   protobuf-c/libprotobuf-c.la '/home/hezhiwen/bin/protobuf-c/lib'
libtool: install: /usr/bin/install -c protobuf-c/.libs/libprotobuf-c.so.1.0.0 /home/hezhiwen/bin/protobuf-c/lib/libprotobuf-c.so.1.0.0
libtool: install: (cd /home/hezhiwen/bin/protobuf-c/lib && { ln -s -f libprotobuf-c.so.1.0.0 libprotobuf-c.so.1 || { rm -f libprotobuf-c.so.1 && ln -s libprotobuf-c.so.1.0.0 libprotobuf-c.so.1; }; })
libtool: install: (cd /home/hezhiwen/bin/protobuf-c/lib && { ln -s -f libprotobuf-c.so.1.0.0 libprotobuf-c.so || { rm -f libprotobuf-c.so && ln -s libprotobuf-c.so.1.0.0 libprotobuf-c.so; }; })
libtool: install: /usr/bin/install -c protobuf-c/.libs/libprotobuf-c.lai /home/hezhiwen/bin/protobuf-c/lib/libprotobuf-c.la
libtool: install: /usr/bin/install -c protobuf-c/.libs/libprotobuf-c.a /home/hezhiwen/bin/protobuf-c/lib/libprotobuf-c.a
libtool: install: chmod 644 /home/hezhiwen/bin/protobuf-c/lib/libprotobuf-c.a
libtool: install: ranlib /home/hezhiwen/bin/protobuf-c/lib/libprotobuf-c.a
libtool: finish: PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/sbin" ldconfig -n /home/hezhiwen/bin/protobuf-c/lib
----------------------------------------------------------------------
Libraries have been installed in:
   /home/hezhiwen/bin/protobuf-c/lib

If you ever happen to want to link against installed libraries
in a given directory, LIBDIR, you must either use libtool, and
specify the full pathname of the library, or use the '-LLIBDIR'
flag during linking and do at least one of the following:
   - add LIBDIR to the 'LD_LIBRARY_PATH' environment variable
     during execution
   - add LIBDIR to the 'LD_RUN_PATH' environment variable
     during linking
   - use the '-Wl,-rpath -Wl,LIBDIR' linker flag
   - have your system administrator add LIBDIR to '/etc/ld.so.conf'

See any operating system documentation about shared libraries for
more information, such as the ld(1) and ld.so(8) manual pages.
----------------------------------------------------------------------
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf-c/bin'
  /bin/bash ./libtool   --mode=install /usr/bin/install -c protoc-c/protoc-gen-c '/home/hezhiwen/bin/protobuf-c/bin'
libtool: install: /usr/bin/install -c protoc-c/protoc-gen-c /home/hezhiwen/bin/protobuf-c/bin/protoc-gen-c
make  install-exec-hook
make[3]: 进入目录“/home/hezhiwen/work/protobuf-c”
rm -f /home/hezhiwen/bin/protobuf-c/bin/protoc-c
ln -s protoc-gen-c /home/hezhiwen/bin/protobuf-c/bin/protoc-c
make[3]: 离开目录“/home/hezhiwen/work/protobuf-c”
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf-c/include'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf-c/include/protobuf-c'
 /usr/bin/install -c -m 644  protobuf-c/protobuf-c.h '/home/hezhiwen/bin/protobuf-c/include/protobuf-c'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf-c/lib/pkgconfig'
 /usr/bin/install -c -m 644 protobuf-c/libprotobuf-c.pc '/home/hezhiwen/bin/protobuf-c/lib/pkgconfig'
make  install-data-hook
make[3]: 进入目录“/home/hezhiwen/work/protobuf-c”
/bin/mkdir -p /home/hezhiwen/bin/protobuf-c/include/google/protobuf-c
cd /home/hezhiwen/bin/protobuf-c/include/google/protobuf-c && rm -f protobuf-c.h
cd /home/hezhiwen/bin/protobuf-c/include/google/protobuf-c && ln -s ../../protobuf-c/protobuf-c.h protobuf-c.h
make[3]: 离开目录“/home/hezhiwen/work/protobuf-c”
make[2]: 离开目录“/home/hezhiwen/work/protobuf-c”
make[1]: 离开目录“/home/hezhiwen/work/protobuf-c”
