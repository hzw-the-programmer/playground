```shell
sudo apt-get install autoconf automake libtool curl make g++ unzip pkg-config

git clone https://github.com/protocolbuffers/protobuf.git
cd protobuf
git submodule update --init --recursive
./autogen.sh

./configure --prefix=/home/hezhiwen/bin/protobuf
make
make check
# sudo make install
# sudo ldconfig
make install
```

# output of ./autogen.sh

+ mkdir -p third_party/googletest/m4
+ autoreconf -f -i -Wall,no-obsolete
libtoolize: putting auxiliary files in AC_CONFIG_AUX_DIR, 'build-aux'.
libtoolize: copying file 'build-aux/ltmain.sh'
libtoolize: putting macros in AC_CONFIG_MACRO_DIRS, 'm4'.
libtoolize: copying file 'm4/libtool.m4'
libtoolize: copying file 'm4/ltoptions.m4'
libtoolize: copying file 'm4/ltsugar.m4'
libtoolize: copying file 'm4/ltversion.m4'
libtoolize: copying file 'm4/lt~obsolete.m4'
configure.ac:27: installing 'build-aux/compile'
configure.ac:30: installing 'build-aux/config.guess'
configure.ac:30: installing 'build-aux/config.sub'
configure.ac:24: installing 'build-aux/install-sh'
configure.ac:24: installing 'build-aux/missing'
Makefile.am: installing 'build-aux/depcomp'
parallel-tests: installing 'build-aux/test-driver'
libtoolize: putting auxiliary files in AC_CONFIG_AUX_DIR, 'build-aux'.
libtoolize: copying file 'build-aux/ltmain.sh'
libtoolize: Consider adding 'AC_CONFIG_MACRO_DIRS([m4])' to configure.ac,
libtoolize: and rerunning libtoolize and aclocal.
libtoolize: Consider adding '-I m4' to ACLOCAL_AMFLAGS in Makefile.am.
configure.ac:22: installing 'build-aux/compile'
configure.ac:25: installing 'build-aux/config.guess'
configure.ac:25: installing 'build-aux/config.sub'
configure.ac:19: installing 'build-aux/install-sh'
configure.ac:19: installing 'build-aux/missing'
Makefile.am: installing 'build-aux/depcomp'
parallel-tests: installing 'build-aux/test-driver'
configure.ac:13: installing 'build-aux/install-sh'
configure.ac:13: installing 'build-aux/missing'
libtoolize: putting auxiliary files in '.'.
libtoolize: copying file './ltmain.sh'
libtoolize: putting macros in AC_CONFIG_MACRO_DIRS, 'm4'.
libtoolize: copying file 'm4/libtool.m4'
libtoolize: copying file 'm4/ltoptions.m4'
libtoolize: copying file 'm4/ltsugar.m4'
libtoolize: copying file 'm4/ltversion.m4'
libtoolize: copying file 'm4/lt~obsolete.m4'
configure.ac:81: installing './ar-lib'
configure.ac:76: installing './compile'
configure.ac:46: installing './config.guess'
configure.ac:46: installing './config.sub'
configure.ac:48: installing './install-sh'
configure.ac:48: installing './missing'
benchmarks/Makefile.am: installing './depcomp'
parallel-tests: installing './test-driver'
+ rm -rf autom4te.cache config.h.in~
+ exit 0

# output of ./configure --prefix=/home/hezhiwen/bin/protobuf

checking whether to enable maintainer-specific portions of Makefiles... yes
checking build system type... x86_64-pc-linux-gnu
checking host system type... x86_64-pc-linux-gnu
checking target system type... x86_64-pc-linux-gnu
checking for a BSD-compatible install... /usr/bin/install -c
checking whether build environment is sane... yes
checking for a thread-safe mkdir -p... /bin/mkdir -p
checking for gawk... no
checking for mawk... mawk
checking whether make sets $(MAKE)... yes
checking whether make supports nested variables... yes
checking whether UID '1000' is supported by ustar format... yes
checking whether GID '1000' is supported by ustar format... yes
checking how to create a ustar tar archive... gnutar
checking whether make supports nested variables... (cached) yes
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
checking for style of include used by make... GNU
checking dependency style of gcc... gcc3
checking for g++... g++
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking dependency style of g++... gcc3
checking how to run the C preprocessor... gcc -E
checking for gcc... gcc
checking whether we are using the GNU C compiler... (cached) yes
checking whether gcc accepts -g... yes
checking for gcc option to accept ISO C89... (cached) none needed
checking whether gcc understands -c and -o together... (cached) yes
checking dependency style of gcc... (cached) gcc3
checking how to run the C preprocessor... gcc -E
checking how to run the C++ preprocessor... g++ -E
checking for g++... g++
checking whether we are using the GNU C++ compiler... (cached) yes
checking whether g++ accepts -g... yes
checking dependency style of g++... (cached) gcc3
checking how to run the C++ preprocessor... g++ -E
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
checking for ar... ar
checking the archiver (ar) interface... ar
checking for gcc... gcc
checking whether we are using the GNU Objective C compiler... no
checking whether gcc accepts -g... no
checking dependency style of gcc... gcc3
checking C++ compiler flags...... use default: -O2  -g -std=c++11 -DNDEBUG
checking whether __SUNPRO_CC is declared... no
checking how to print strings... printf
checking for a sed that does not truncate output... /bin/sed
checking for fgrep... /bin/grep -F
checking for ld used by gcc... /usr/bin/ld
checking if the linker (/usr/bin/ld) is GNU ld... yes
checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
checking the name lister (/usr/bin/nm -B) interface... BSD nm
checking whether ln -s works... yes
checking the maximum length of command line arguments... 1572864
checking how to convert x86_64-pc-linux-gnu file names to x86_64-pc-linux-gnu format... func_convert_file_noop
checking how to convert x86_64-pc-linux-gnu file names to toolchain format... func_convert_file_noop
checking for /usr/bin/ld option to reload object files... -r
checking for objdump... objdump
checking how to recognize dependent libraries... pass_all
checking for dlltool... no
checking how to associate runtime and link libraries... printf %s\n
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
checking whether the linker supports version scripts... yes
checking for ANSI C header files... (cached) yes
checking fcntl.h usability... yes
checking fcntl.h presence... yes
checking for fcntl.h... yes
checking for inttypes.h... (cached) yes
checking limits.h usability... yes
checking limits.h presence... yes
checking for limits.h... yes
checking for stdlib.h... (cached) yes
checking for unistd.h... (cached) yes
checking for working memcmp... yes
checking for working strtod... yes
checking for ftruncate... yes
checking for memset... yes
checking for mkdir... yes
checking for strchr... yes
checking for strerror... yes
checking for strtol... yes
checking zlib version... headers missing or too old (requires 1.2.0.4)
checking whether g++ supports C++11 features by default... yes
checking whether -latomic is needed... no
checking whether gcc is Clang... no
checking whether pthreads work with -pthread... yes
checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
checking whether more special flags are required for pthreads... no
checking for PTHREAD_PRIO_INHERIT... yes
checking the location of hash_map... <unordered_map>
checking that generated files are newer than configure... done
configure: creating ./config.status
config.status: creating Makefile
config.status: creating src/Makefile
config.status: creating benchmarks/Makefile
config.status: creating conformance/Makefile
config.status: creating protobuf.pc
config.status: creating protobuf-lite.pc
config.status: creating config.h
config.status: executing depfiles commands
config.status: executing libtool commands
=== configuring in third_party/googletest (/home/hezhiwen/work/protobuf/third_party/googletest)
configure: running /bin/bash ./configure --disable-option-checking '--prefix=/home/hezhiwen/bin/protobuf'  --cache-file=/dev/null --srcdir=.
checking for a BSD-compatible install... /usr/bin/install -c
checking whether build environment is sane... yes
checking for a thread-safe mkdir -p... /bin/mkdir -p
checking for gawk... no
checking for mawk... mawk
checking whether make sets $(MAKE)... yes
checking whether make supports nested variables... yes
checking that generated files are newer than configure... done
configure: creating ./config.status
config.status: creating Makefile
=== configuring in googletest (/home/hezhiwen/work/protobuf/third_party/googletest/googletest)
configure: running /bin/bash ./configure --disable-option-checking '--prefix=/home/hezhiwen/bin/protobuf'  --cache-file=/dev/null --srcdir=.
checking for a BSD-compatible install... /usr/bin/install -c
checking whether build environment is sane... yes
checking for a thread-safe mkdir -p... /bin/mkdir -p
checking for gawk... no
checking for mawk... mawk
checking whether make sets $(MAKE)... yes
checking whether make supports nested variables... yes
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
checking for style of include used by make... GNU
checking dependency style of gcc... gcc3
checking for g++... g++
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking dependency style of g++... gcc3
checking build system type... x86_64-pc-linux-gnu
checking host system type... x86_64-pc-linux-gnu
checking how to print strings... printf
checking for a sed that does not truncate output... /bin/sed
checking for grep that handles long lines and -e... /bin/grep
checking for egrep... /bin/grep -E
checking for fgrep... /bin/grep -F
checking for ld used by gcc... /usr/bin/ld
checking if the linker (/usr/bin/ld) is GNU ld... yes
checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
checking the name lister (/usr/bin/nm -B) interface... BSD nm
checking whether ln -s works... yes
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
checking how to run the C preprocessor... gcc -E
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
checking for python... :
checking for the pthreads library -lpthreads... no
checking whether pthreads work without any flags... no
checking whether pthreads work with -Kthread... no
checking whether pthreads work with -kthread... no
checking for the pthreads library -llthread... no
checking whether pthreads work with -pthread... yes
checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
checking if more special flags are required for pthreads... no
checking whether to check for GCC pthread/shared inconsistencies... yes
checking whether -pthread is sufficient with -shared... yes
checking that generated files are newer than configure... done
configure: creating ./config.status
config.status: creating Makefile
config.status: creating scripts/gtest-config
config.status: creating build-aux/config.h
config.status: executing depfiles commands
config.status: executing libtool commands
=== configuring in googlemock (/home/hezhiwen/work/protobuf/third_party/googletest/googlemock)
configure: running /bin/bash ./configure --disable-option-checking '--prefix=/home/hezhiwen/bin/protobuf'  --cache-file=/dev/null --srcdir=.
checking for a BSD-compatible install... /usr/bin/install -c
checking whether build environment is sane... yes
checking for a thread-safe mkdir -p... /bin/mkdir -p
checking for gawk... no
checking for mawk... mawk
checking whether make sets $(MAKE)... yes
checking whether make supports nested variables... yes
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
checking for style of include used by make... GNU
checking dependency style of gcc... gcc3
checking for g++... g++
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking dependency style of g++... gcc3
checking build system type... x86_64-pc-linux-gnu
checking host system type... x86_64-pc-linux-gnu
checking how to print strings... printf
checking for a sed that does not truncate output... /bin/sed
checking for grep that handles long lines and -e... /bin/grep
checking for egrep... /bin/grep -E
checking for fgrep... /bin/grep -F
checking for ld used by gcc... /usr/bin/ld
checking if the linker (/usr/bin/ld) is GNU ld... yes
checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
checking the name lister (/usr/bin/nm -B) interface... BSD nm
checking whether ln -s works... yes
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
checking how to run the C preprocessor... gcc -E
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
checking for python... :
checking for the pthreads library -lpthreads... no
checking whether pthreads work without any flags... no
checking whether pthreads work with -Kthread... no
checking whether pthreads work with -kthread... no
checking for the pthreads library -llthread... no
checking whether pthreads work with -pthread... yes
checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
checking if more special flags are required for pthreads... no
checking whether to check for GCC pthread/shared inconsistencies... yes
checking whether -pthread is sufficient with -shared... yes
checking for gtest-config... no
checking that generated files are newer than configure... done
configure: creating ./config.status
config.status: creating Makefile
config.status: creating scripts/gmock-config
config.status: creating build-aux/config.h
config.status: executing depfiles commands
config.status: executing libtool commands

# output of make

make  all-recursive
make[1]: 进入目录“/home/hezhiwen/work/protobuf”
Making all in .
make[2]: 进入目录“/home/hezhiwen/work/protobuf”
make[2]: 离开目录“/home/hezhiwen/work/protobuf”
Making all in src
make[2]: 进入目录“/home/hezhiwen/work/protobuf/src”
  CXX      google/protobuf/stubs/bytestream.lo
  CXX      google/protobuf/stubs/common.lo
  CXX      google/protobuf/stubs/int128.lo
  CXX      google/protobuf/io/io_win32.lo
  CXX      google/protobuf/stubs/status.lo
  CXX      google/protobuf/stubs/statusor.lo
  CXX      google/protobuf/stubs/stringpiece.lo
  CXX      google/protobuf/stubs/stringprintf.lo
  CXX      google/protobuf/stubs/structurally_valid.lo
  CXX      google/protobuf/stubs/strutil.lo
  CXX      google/protobuf/stubs/time.lo
  CXX      google/protobuf/any_lite.lo
  CXX      google/protobuf/arena.lo
  CXX      google/protobuf/extension_set.lo
  CXX      google/protobuf/generated_enum_util.lo
  CXX      google/protobuf/generated_message_util.lo
  CXX      google/protobuf/generated_message_table_driven_lite.lo
  CXX      google/protobuf/implicit_weak_message.lo
  CXX      google/protobuf/message_lite.lo
In file included from /usr/include/string.h:494:0,
                 from ./google/protobuf/stubs/port.h:39,
                 from ./google/protobuf/stubs/common.h:46,
                 from ./google/protobuf/message_lite.h:45,
                 from google/protobuf/message_lite.cc:36:
In function ‘void* memcpy(void*, const void*, size_t)’,
    inlined from ‘google::protobuf::uint8* google::protobuf::io::EpsCopyOutputStream::WriteRaw(const void*, int, google::protobuf::uint8*)’ at ./google/protobuf/io/coded_stream.h:699:16,
    inlined from ‘virtual google::protobuf::uint8* google::protobuf::internal::ImplicitWeakMessage::_InternalSerialize(google::protobuf::uint8*, google::protobuf::io::EpsCopyOutputStream*) const’ at ./google/protobuf/implicit_weak_message.h:86:35,
    inlined from ‘bool google::protobuf::MessageLite::SerializePartialToZeroCopyStream(google::protobuf::io::ZeroCopyOutputStream*) const’ at google/protobuf/message_lite.cc:419:30:
/usr/include/x86_64-linux-gnu/bits/string_fortified.h:34:71: warning: ‘void* __builtin___memcpy_chk(void*, const void*, long unsigned int, long unsigned int)’: specified size between 18446744071562067968 and 18446744073709551615 exceeds maximum object size 9223372036854775807 [-Wstringop-overflow=]
   return __builtin___memcpy_chk (__dest, __src, __len, __bos0 (__dest));
                                                                       ^
  CXX      google/protobuf/parse_context.lo
  CXX      google/protobuf/repeated_field.lo
  CXX      google/protobuf/wire_format_lite.lo
  CXX      google/protobuf/io/coded_stream.lo
  CXX      google/protobuf/io/strtod.lo
  CXX      google/protobuf/io/zero_copy_stream.lo
  CXX      google/protobuf/io/zero_copy_stream_impl.lo
  CXX      google/protobuf/io/zero_copy_stream_impl_lite.lo
  CXXLD    libprotobuf-lite.la
ar: `u' modifier ignored since `D' is the default (see `U')
  CXX      google/protobuf/any.pb.lo
  CXX      google/protobuf/api.pb.lo
  CXX      google/protobuf/any.lo
  CXX      google/protobuf/descriptor.lo
  CXX      google/protobuf/descriptor_database.lo
  CXX      google/protobuf/descriptor.pb.lo
  CXX      google/protobuf/duration.pb.lo
  CXX      google/protobuf/dynamic_message.lo
  CXX      google/protobuf/empty.pb.lo
  CXX      google/protobuf/extension_set_heavy.lo
  CXX      google/protobuf/field_mask.pb.lo
  CXX      google/protobuf/generated_message_reflection.lo
  CXX      google/protobuf/generated_message_table_driven.lo
  CXX      google/protobuf/map_field.lo
  CXX      google/protobuf/message.lo
  CXX      google/protobuf/reflection_ops.lo
  CXX      google/protobuf/service.lo
  CXX      google/protobuf/source_context.pb.lo
  CXX      google/protobuf/struct.pb.lo
  CXX      google/protobuf/stubs/substitute.lo
  CXX      google/protobuf/text_format.lo
  CXX      google/protobuf/timestamp.pb.lo
  CXX      google/protobuf/type.pb.lo
  CXX      google/protobuf/unknown_field_set.lo
  CXX      google/protobuf/wire_format.lo
  CXX      google/protobuf/wrappers.pb.lo
  CXX      google/protobuf/io/gzip_stream.lo
  CXX      google/protobuf/io/printer.lo
  CXX      google/protobuf/io/tokenizer.lo
  CXX      google/protobuf/compiler/importer.lo
  CXX      google/protobuf/compiler/parser.lo
  CXX      google/protobuf/util/delimited_message_util.lo
  CXX      google/protobuf/util/field_comparator.lo
  CXX      google/protobuf/util/field_mask_util.lo
  CXX      google/protobuf/util/internal/datapiece.lo
  CXX      google/protobuf/util/internal/default_value_objectwriter.lo
  CXX      google/protobuf/util/internal/error_listener.lo
  CXX      google/protobuf/util/internal/field_mask_utility.lo
  CXX      google/protobuf/util/internal/json_escaping.lo
  CXX      google/protobuf/util/internal/json_objectwriter.lo
  CXX      google/protobuf/util/internal/json_stream_parser.lo
  CXX      google/protobuf/util/internal/object_writer.lo
  CXX      google/protobuf/util/internal/protostream_objectsource.lo
  CXX      google/protobuf/util/internal/protostream_objectwriter.lo
  CXX      google/protobuf/util/internal/proto_writer.lo
  CXX      google/protobuf/util/internal/type_info.lo
  CXX      google/protobuf/util/internal/type_info_test_helper.lo
  CXX      google/protobuf/util/internal/utility.lo
  CXX      google/protobuf/util/json_util.lo
  CXX      google/protobuf/util/message_differencer.lo
  CXX      google/protobuf/util/time_util.lo
  CXX      google/protobuf/util/type_resolver_util.lo
  CXXLD    libprotobuf.la
ar: `u' modifier ignored since `D' is the default (see `U')
  CXX      google/protobuf/compiler/code_generator.lo
  CXX      google/protobuf/compiler/command_line_interface.lo
  CXX      google/protobuf/compiler/plugin.lo
  CXX      google/protobuf/compiler/plugin.pb.lo
  CXX      google/protobuf/compiler/subprocess.lo
  CXX      google/protobuf/compiler/zip_writer.lo
  CXX      google/protobuf/compiler/cpp/cpp_enum.lo
  CXX      google/protobuf/compiler/cpp/cpp_enum_field.lo
  CXX      google/protobuf/compiler/cpp/cpp_extension.lo
  CXX      google/protobuf/compiler/cpp/cpp_field.lo
  CXX      google/protobuf/compiler/cpp/cpp_file.lo
  CXX      google/protobuf/compiler/cpp/cpp_generator.lo
  CXX      google/protobuf/compiler/cpp/cpp_helpers.lo
  CXX      google/protobuf/compiler/cpp/cpp_map_field.lo
  CXX      google/protobuf/compiler/cpp/cpp_message.lo
  CXX      google/protobuf/compiler/cpp/cpp_message_field.lo
  CXX      google/protobuf/compiler/cpp/cpp_padding_optimizer.lo
  CXX      google/protobuf/compiler/cpp/cpp_primitive_field.lo
  CXX      google/protobuf/compiler/cpp/cpp_service.lo
  CXX      google/protobuf/compiler/cpp/cpp_string_field.lo
  CXX      google/protobuf/compiler/java/java_context.lo
  CXX      google/protobuf/compiler/java/java_enum.lo
  CXX      google/protobuf/compiler/java/java_enum_lite.lo
  CXX      google/protobuf/compiler/java/java_enum_field.lo
  CXX      google/protobuf/compiler/java/java_enum_field_lite.lo
  CXX      google/protobuf/compiler/java/java_extension.lo
  CXX      google/protobuf/compiler/java/java_extension_lite.lo
  CXX      google/protobuf/compiler/java/java_field.lo
  CXX      google/protobuf/compiler/java/java_file.lo
  CXX      google/protobuf/compiler/java/java_generator.lo
  CXX      google/protobuf/compiler/java/java_generator_factory.lo
  CXX      google/protobuf/compiler/java/java_helpers.lo
  CXX      google/protobuf/compiler/java/java_map_field.lo
  CXX      google/protobuf/compiler/java/java_map_field_lite.lo
  CXX      google/protobuf/compiler/java/java_message.lo
  CXX      google/protobuf/compiler/java/java_message_lite.lo
  CXX      google/protobuf/compiler/java/java_message_builder.lo
  CXX      google/protobuf/compiler/java/java_message_builder_lite.lo
  CXX      google/protobuf/compiler/java/java_message_field.lo
  CXX      google/protobuf/compiler/java/java_message_field_lite.lo
  CXX      google/protobuf/compiler/java/java_name_resolver.lo
  CXX      google/protobuf/compiler/java/java_primitive_field.lo
  CXX      google/protobuf/compiler/java/java_primitive_field_lite.lo
  CXX      google/protobuf/compiler/java/java_shared_code_generator.lo
  CXX      google/protobuf/compiler/java/java_service.lo
  CXX      google/protobuf/compiler/java/java_string_field.lo
  CXX      google/protobuf/compiler/java/java_string_field_lite.lo
  CXX      google/protobuf/compiler/java/java_doc_comment.lo
  CXX      google/protobuf/compiler/js/js_generator.lo
  CXX      google/protobuf/compiler/js/well_known_types_embed.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_enum.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_enum_field.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_extension.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_field.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_file.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_generator.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_helpers.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_map_field.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_message.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_message_field.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_oneof.lo
  CXX      google/protobuf/compiler/objectivec/objectivec_primitive_field.lo
  CXX      google/protobuf/compiler/php/php_generator.lo
  CXX      google/protobuf/compiler/python/python_generator.lo
  CXX      google/protobuf/compiler/ruby/ruby_generator.lo
  CXX      google/protobuf/compiler/csharp/csharp_doc_comment.lo
  CXX      google/protobuf/compiler/csharp/csharp_enum.lo
  CXX      google/protobuf/compiler/csharp/csharp_enum_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_field_base.lo
  CXX      google/protobuf/compiler/csharp/csharp_generator.lo
  CXX      google/protobuf/compiler/csharp/csharp_helpers.lo
  CXX      google/protobuf/compiler/csharp/csharp_map_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_message.lo
  CXX      google/protobuf/compiler/csharp/csharp_message_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_primitive_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_reflection_class.lo
  CXX      google/protobuf/compiler/csharp/csharp_repeated_enum_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_repeated_message_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_repeated_primitive_field.lo
  CXX      google/protobuf/compiler/csharp/csharp_source_generator_base.lo
  CXX      google/protobuf/compiler/csharp/csharp_wrapper_field.lo
  CXXLD    libprotoc.la
ar: `u' modifier ignored since `D' is the default (see `U')
  CXX      google/protobuf/compiler/main.o
  CXXLD    protoc
make[2]: 离开目录“/home/hezhiwen/work/protobuf/src”
make[1]: 离开目录“/home/hezhiwen/work/protobuf”

# output of make check

Making check in .
make[1]: 进入目录“/home/hezhiwen/work/protobuf”
make  check-local
make[2]: 进入目录“/home/hezhiwen/work/protobuf”
Making lib/libgmock.a lib/libgmock_main.a in gmock
make[3]: 进入目录“/home/hezhiwen/work/protobuf/third_party/googletest/googletest”
depbase=`echo src/gtest-all.lo | sed 's|[^/]*$|.deps/&|;s|\.lo$||'`;\
/bin/bash ./libtool  --tag=CXX   --mode=compile g++ -DHAVE_CONFIG_H -I. -I./build-aux  -I. -I./include  -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gtest-all.lo -MD -MP -MF $depbase.Tpo -c -o src/gtest-all.lo src/gtest-all.cc &&\
mv -f $depbase.Tpo $depbase.Plo
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I. -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gtest-all.lo -MD -MP -MF src/.deps/gtest-all.Tpo -c src/gtest-all.cc  -fPIC -DPIC -o src/.libs/gtest-all.o
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I. -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gtest-all.lo -MD -MP -MF src/.deps/gtest-all.Tpo -c src/gtest-all.cc -o src/gtest-all.o >/dev/null 2>&1
/bin/bash ./libtool  --tag=CXX   --mode=link g++ -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG   -o lib/libgtest.la -rpath /home/hezhiwen/bin/protobuf/lib src/gtest-all.lo  
libtool: link: g++  -fPIC -DPIC -shared -nostdlib /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crti.o /usr/lib/gcc/x86_64-linux-gnu/7/crtbeginS.o  src/.libs/gtest-all.o   -L/usr/lib/gcc/x86_64-linux-gnu/7 -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../../lib -L/lib/x86_64-linux-gnu -L/lib/../lib -L/usr/lib/x86_64-linux-gnu -L/usr/lib/../lib -L/usr/lib/gcc/x86_64-linux-gnu/7/../../.. -lstdc++ -lm -lc -lgcc_s /usr/lib/gcc/x86_64-linux-gnu/7/crtendS.o /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crtn.o  -pthread -g   -pthread -Wl,-soname -Wl,libgtest.so.0 -o lib/.libs/libgtest.so.0.0.0
libtool: link: (cd "lib/.libs" && rm -f "libgtest.so.0" && ln -s "libgtest.so.0.0.0" "libgtest.so.0")
libtool: link: (cd "lib/.libs" && rm -f "libgtest.so" && ln -s "libgtest.so.0.0.0" "libgtest.so")
libtool: link: ar cru lib/.libs/libgtest.a  src/gtest-all.o
ar: `u' modifier ignored since `D' is the default (see `U')
libtool: link: ranlib lib/.libs/libgtest.a
libtool: link: ( cd "lib/.libs" && rm -f "libgtest.la" && ln -s "../libgtest.la" "libgtest.la" )
depbase=`echo src/gtest_main.lo | sed 's|[^/]*$|.deps/&|;s|\.lo$||'`;\
/bin/bash ./libtool  --tag=CXX   --mode=compile g++ -DHAVE_CONFIG_H -I. -I./build-aux  -I. -I./include  -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gtest_main.lo -MD -MP -MF $depbase.Tpo -c -o src/gtest_main.lo src/gtest_main.cc &&\
mv -f $depbase.Tpo $depbase.Plo
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I. -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gtest_main.lo -MD -MP -MF src/.deps/gtest_main.Tpo -c src/gtest_main.cc  -fPIC -DPIC -o src/.libs/gtest_main.o
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I. -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gtest_main.lo -MD -MP -MF src/.deps/gtest_main.Tpo -c src/gtest_main.cc -o src/gtest_main.o >/dev/null 2>&1
/bin/bash ./libtool  --tag=CXX   --mode=link g++ -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG   -o lib/libgtest_main.la -rpath /home/hezhiwen/bin/protobuf/lib src/gtest_main.lo lib/libgtest.la 
libtool: link: g++  -fPIC -DPIC -shared -nostdlib /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crti.o /usr/lib/gcc/x86_64-linux-gnu/7/crtbeginS.o  src/.libs/gtest_main.o   -Wl,-rpath -Wl,/home/hezhiwen/work/protobuf/third_party/googletest/googletest/lib/.libs -Wl,-rpath -Wl,/home/hezhiwen/bin/protobuf/lib lib/.libs/libgtest.so -L/usr/lib/gcc/x86_64-linux-gnu/7 -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../../lib -L/lib/x86_64-linux-gnu -L/lib/../lib -L/usr/lib/x86_64-linux-gnu -L/usr/lib/../lib -L/usr/lib/gcc/x86_64-linux-gnu/7/../../.. -lstdc++ -lm -lc -lgcc_s /usr/lib/gcc/x86_64-linux-gnu/7/crtendS.o /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crtn.o  -pthread -g   -pthread -Wl,-soname -Wl,libgtest_main.so.0 -o lib/.libs/libgtest_main.so.0.0.0
libtool: link: (cd "lib/.libs" && rm -f "libgtest_main.so.0" && ln -s "libgtest_main.so.0.0.0" "libgtest_main.so.0")
libtool: link: (cd "lib/.libs" && rm -f "libgtest_main.so" && ln -s "libgtest_main.so.0.0.0" "libgtest_main.so")
libtool: link: ar cru lib/.libs/libgtest_main.a  src/gtest_main.o
ar: `u' modifier ignored since `D' is the default (see `U')
libtool: link: ranlib lib/.libs/libgtest_main.a
libtool: link: ( cd "lib/.libs" && rm -f "libgtest_main.la" && ln -s "../libgtest_main.la" "libgtest_main.la" )
make[3]: 离开目录“/home/hezhiwen/work/protobuf/third_party/googletest/googletest”
make[3]: 进入目录“/home/hezhiwen/work/protobuf/third_party/googletest/googlemock”
depbase=`echo src/gmock-all.lo | sed 's|[^/]*$|.deps/&|;s|\.lo$||'`;\
/bin/bash ./libtool  --tag=CXX   --mode=compile g++ -DHAVE_CONFIG_H -I. -I./build-aux  -I./../googletest/include -I./include  -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gmock-all.lo -MD -MP -MF $depbase.Tpo -c -o src/gmock-all.lo src/gmock-all.cc &&\
mv -f $depbase.Tpo $depbase.Plo
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I./../googletest/include -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gmock-all.lo -MD -MP -MF src/.deps/gmock-all.Tpo -c src/gmock-all.cc  -fPIC -DPIC -o src/.libs/gmock-all.o
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I./../googletest/include -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gmock-all.lo -MD -MP -MF src/.deps/gmock-all.Tpo -c src/gmock-all.cc -o src/gmock-all.o >/dev/null 2>&1
/bin/bash ./libtool  --tag=CXX   --mode=link g++ -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG   -o lib/libgmock.la -rpath /home/hezhiwen/bin/protobuf/lib src/gmock-all.lo  
libtool: link: g++  -fPIC -DPIC -shared -nostdlib /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crti.o /usr/lib/gcc/x86_64-linux-gnu/7/crtbeginS.o  src/.libs/gmock-all.o   -L/usr/lib/gcc/x86_64-linux-gnu/7 -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../../lib -L/lib/x86_64-linux-gnu -L/lib/../lib -L/usr/lib/x86_64-linux-gnu -L/usr/lib/../lib -L/usr/lib/gcc/x86_64-linux-gnu/7/../../.. -lstdc++ -lm -lc -lgcc_s /usr/lib/gcc/x86_64-linux-gnu/7/crtendS.o /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crtn.o  -pthread -g   -pthread -Wl,-soname -Wl,libgmock.so.0 -o lib/.libs/libgmock.so.0.0.0
libtool: link: (cd "lib/.libs" && rm -f "libgmock.so.0" && ln -s "libgmock.so.0.0.0" "libgmock.so.0")
libtool: link: (cd "lib/.libs" && rm -f "libgmock.so" && ln -s "libgmock.so.0.0.0" "libgmock.so")
libtool: link: ar cru lib/.libs/libgmock.a  src/gmock-all.o
ar: `u' modifier ignored since `D' is the default (see `U')
libtool: link: ranlib lib/.libs/libgmock.a
libtool: link: ( cd "lib/.libs" && rm -f "libgmock.la" && ln -s "../libgmock.la" "libgmock.la" )
depbase=`echo src/gmock_main.lo | sed 's|[^/]*$|.deps/&|;s|\.lo$||'`;\
/bin/bash ./libtool  --tag=CXX   --mode=compile g++ -DHAVE_CONFIG_H -I. -I./build-aux  -I./../googletest/include -I./include  -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gmock_main.lo -MD -MP -MF $depbase.Tpo -c -o src/gmock_main.lo src/gmock_main.cc &&\
mv -f $depbase.Tpo $depbase.Plo
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I./../googletest/include -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gmock_main.lo -MD -MP -MF src/.deps/gmock_main.Tpo -c src/gmock_main.cc  -fPIC -DPIC -o src/.libs/gmock_main.o
libtool: compile:  g++ -DHAVE_CONFIG_H -I. -I./build-aux -I./../googletest/include -I./include -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG -MT src/gmock_main.lo -MD -MP -MF src/.deps/gmock_main.Tpo -c src/gmock_main.cc -o src/gmock_main.o >/dev/null 2>&1
/bin/bash ./libtool  --tag=CXX   --mode=link g++ -pthread -DGTEST_HAS_PTHREAD=1 -g -std=c++11 -DNDEBUG   -o lib/libgmock_main.la -rpath /home/hezhiwen/bin/protobuf/lib src/gmock_main.lo lib/libgmock.la 
libtool: link: g++  -fPIC -DPIC -shared -nostdlib /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crti.o /usr/lib/gcc/x86_64-linux-gnu/7/crtbeginS.o  src/.libs/gmock_main.o   -Wl,-rpath -Wl,/home/hezhiwen/work/protobuf/third_party/googletest/googlemock/lib/.libs -Wl,-rpath -Wl,/home/hezhiwen/bin/protobuf/lib lib/.libs/libgmock.so -L/usr/lib/gcc/x86_64-linux-gnu/7 -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu -L/usr/lib/gcc/x86_64-linux-gnu/7/../../../../lib -L/lib/x86_64-linux-gnu -L/lib/../lib -L/usr/lib/x86_64-linux-gnu -L/usr/lib/../lib -L/usr/lib/gcc/x86_64-linux-gnu/7/../../.. -lstdc++ -lm -lc -lgcc_s /usr/lib/gcc/x86_64-linux-gnu/7/crtendS.o /usr/lib/gcc/x86_64-linux-gnu/7/../../../x86_64-linux-gnu/crtn.o  -pthread -g   -pthread -Wl,-soname -Wl,libgmock_main.so.0 -o lib/.libs/libgmock_main.so.0.0.0
libtool: link: (cd "lib/.libs" && rm -f "libgmock_main.so.0" && ln -s "libgmock_main.so.0.0.0" "libgmock_main.so.0")
libtool: link: (cd "lib/.libs" && rm -f "libgmock_main.so" && ln -s "libgmock_main.so.0.0.0" "libgmock_main.so")
libtool: link: ar cru lib/.libs/libgmock_main.a  src/gmock_main.o
ar: `u' modifier ignored since `D' is the default (see `U')
libtool: link: ranlib lib/.libs/libgmock_main.a
libtool: link: ( cd "lib/.libs" && rm -f "libgmock_main.la" && ln -s "../libgmock_main.la" "libgmock_main.la" )
make[3]: 离开目录“/home/hezhiwen/work/protobuf/third_party/googletest/googlemock”
make[2]: 离开目录“/home/hezhiwen/work/protobuf”
make[1]: 离开目录“/home/hezhiwen/work/protobuf”
Making check in src
make[1]: 进入目录“/home/hezhiwen/work/protobuf/src”
make  protoc protobuf-test protobuf-lazy-descriptor-test protobuf-lite-test test_plugin protobuf-lite-arena-test no-warning-test 
make[2]: 进入目录“/home/hezhiwen/work/protobuf/src”
make[2]: “protoc”已是最新。
oldpwd=`pwd` && ( cd . && $oldpwd/protoc -I. --cpp_out=$oldpwd google/protobuf/any_test.proto google/protobuf/compiler/cpp/cpp_test_bad_identifiers.proto google/protobuf/map_lite_unittest.proto google/protobuf/map_proto2_unittest.proto google/protobuf/map_unittest.proto google/protobuf/unittest_arena.proto google/protobuf/unittest_custom_options.proto google/protobuf/unittest_drop_unknown_fields.proto google/protobuf/unittest_embed_optimize_for.proto google/protobuf/unittest_empty.proto google/protobuf/unittest_enormous_descriptor.proto google/protobuf/unittest_import_lite.proto google/protobuf/unittest_import.proto google/protobuf/unittest_import_public_lite.proto google/protobuf/unittest_import_public.proto google/protobuf/unittest_lazy_dependencies.proto google/protobuf/unittest_lazy_dependencies_custom_option.proto google/protobuf/unittest_lazy_dependencies_enum.proto google/protobuf/unittest_lite_imports_nonlite.proto google/protobuf/unittest_lite.proto google/protobuf/unittest_mset.proto google/protobuf/unittest_mset_wire_format.proto google/protobuf/unittest_no_arena_lite.proto google/protobuf/unittest_no_arena_import.proto google/protobuf/unittest_no_arena.proto google/protobuf/unittest_no_field_presence.proto google/protobuf/unittest_no_generic_services.proto google/protobuf/unittest_optimize_for.proto google/protobuf/unittest_preserve_unknown_enum2.proto google/protobuf/unittest_preserve_unknown_enum.proto google/protobuf/unittest.proto google/protobuf/unittest_proto3.proto google/protobuf/unittest_proto3_arena.proto google/protobuf/unittest_proto3_arena_lite.proto google/protobuf/unittest_proto3_lite.proto google/protobuf/unittest_proto3_optional.proto google/protobuf/unittest_well_known_types.proto google/protobuf/util/internal/testdata/anys.proto google/protobuf/util/internal/testdata/books.proto google/protobuf/util/internal/testdata/default_value.proto google/protobuf/util/internal/testdata/default_value_test.proto google/protobuf/util/internal/testdata/field_mask.proto google/protobuf/util/internal/testdata/maps.proto google/protobuf/util/internal/testdata/oneofs.proto google/protobuf/util/internal/testdata/proto3.proto google/protobuf/util/internal/testdata/struct.proto google/protobuf/util/internal/testdata/timestamp_duration.proto google/protobuf/util/internal/testdata/wrappers.proto google/protobuf/util/json_format.proto google/protobuf/util/json_format_proto3.proto google/protobuf/util/message_differencer_unittest.proto google/protobuf/compiler/cpp/cpp_test_large_enum_value.proto --experimental_allow_proto3_optional )
touch unittest_proto_middleman
  CXX      google/protobuf/stubs/protobuf_test-bytestream_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-common_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-int128_unittest.o
  CXX      google/protobuf/io/protobuf_test-io_win32_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-statusor_test.o
  CXX      google/protobuf/stubs/protobuf_test-status_test.o
  CXX      google/protobuf/stubs/protobuf_test-stringpiece_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-stringprintf_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-structurally_valid_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-strutil_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-template_util_unittest.o
  CXX      google/protobuf/stubs/protobuf_test-time_test.o
  CXX      google/protobuf/protobuf_test-any_test.o
  CXX      google/protobuf/protobuf_test-arenastring_unittest.o
  CXX      google/protobuf/protobuf_test-arena_unittest.o
google/protobuf/arena_unittest.cc: In member function ‘virtual void google::protobuf::ArenaTest_UnsafeArenaReleaseAdd_Test::TestBody()’:
google/protobuf/arena_unittest.cc:887:68: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_optional_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message1->unsafe_arena_set_allocated_optional_string(arena_string);
                                                                    ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21521:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_optional_string(
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:889:54: warning: ‘std::__cxx11::string* protobuf_unittest::TestAllTypes::unsafe_arena_release_optional_string()’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
       message1->unsafe_arena_release_optional_string());
                                                      ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21514:21: note: declared here
 inline std::string* TestAllTypes::unsafe_arena_release_optional_string() {
                     ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:889:55: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_optional_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
       message1->unsafe_arena_release_optional_string());
                                                       ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21521:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_optional_string(
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc: In member function ‘virtual void google::protobuf::ArenaTest_UnsafeArenaRelease_Test::TestBody()’:
google/protobuf/arena_unittest.cc:908:56: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_optional_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message->unsafe_arena_set_allocated_optional_string(s);
                                                        ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21521:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_optional_string(
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:911:53: warning: ‘std::__cxx11::string* protobuf_unittest::TestAllTypes::unsafe_arena_release_optional_string()’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   s = message->unsafe_arena_release_optional_string();
                                                     ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21514:21: note: declared here
 inline std::string* TestAllTypes::unsafe_arena_release_optional_string() {
                     ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:916:53: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_oneof_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message->unsafe_arena_set_allocated_oneof_string(s);
                                                     ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:24661:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_oneof_string(std::string* oneof_string) {
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:919:50: warning: ‘std::__cxx11::string* protobuf_unittest::TestAllTypes::unsafe_arena_release_oneof_string()’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   s = message->unsafe_arena_release_oneof_string();
                                                  ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:24650:21: note: declared here
 inline std::string* TestAllTypes::unsafe_arena_release_oneof_string() {
                     ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc: In member function ‘virtual void google::protobuf::ArenaTest_OneofMerge_Test::TestBody()’:
google/protobuf/arena_unittest.cc:929:73: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_oneof_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message0->unsafe_arena_set_allocated_oneof_string(new std::string("x"));
                                                                         ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:24661:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_oneof_string(std::string* oneof_string) {
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:931:73: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_oneof_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message1->unsafe_arena_set_allocated_oneof_string(new std::string("y"));
                                                                         ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:24661:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_oneof_string(std::string* oneof_string) {
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:938:54: warning: ‘std::__cxx11::string* protobuf_unittest::TestAllTypes::unsafe_arena_release_oneof_string()’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   delete message0->unsafe_arena_release_oneof_string();
                                                      ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:24650:21: note: declared here
 inline std::string* TestAllTypes::unsafe_arena_release_oneof_string() {
                     ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:939:54: warning: ‘std::__cxx11::string* protobuf_unittest::TestAllTypes::unsafe_arena_release_oneof_string()’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   delete message1->unsafe_arena_release_oneof_string();
                                                      ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:24650:21: note: declared here
 inline std::string* TestAllTypes::unsafe_arena_release_oneof_string() {
                     ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc: In member function ‘virtual void google::protobuf::ArenaTest_UnsafeSetAllocatedOnArena_Test::TestBody()’:
google/protobuf/arena_unittest.cc:1374:68: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_optional_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message->unsafe_arena_set_allocated_optional_string(&owned_string);
                                                                    ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21521:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_optional_string(
             ^~~~~~~~~~~~
google/protobuf/arena_unittest.cc:1377:59: warning: ‘void protobuf_unittest::TestAllTypes::unsafe_arena_set_allocated_optional_string(std::__cxx11::string*)’ is deprecated: The unsafe_arena_ accessors for    string fields are deprecated and will be removed in a    future release. [-Wdeprecated-declarations]
   message->unsafe_arena_set_allocated_optional_string(NULL);
                                                           ^
In file included from ./google/protobuf/test_util.h:38:0,
                 from google/protobuf/arena_unittest.cc:44:
./google/protobuf/unittest.pb.h:21521:13: note: declared here
 inline void TestAllTypes::unsafe_arena_set_allocated_optional_string(
             ^~~~~~~~~~~~
  CXX      google/protobuf/protobuf_test-descriptor_database_unittest.o
  CXX      google/protobuf/protobuf_test-descriptor_unittest.o
  CXX      google/protobuf/protobuf_test-drop_unknown_fields_test.o
  CXX      google/protobuf/protobuf_test-dynamic_message_unittest.o
google/protobuf/dynamic_message_unittest.cc: In member function ‘virtual void google::protobuf::DynamicMessageTest_SpaceUsed_Test::TestBody()’:
google/protobuf/dynamic_message_unittest.cc:264:47: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   int initial_space_used = message->SpaceUsed();
                                               ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/dynamic_message_unittest.cc:45:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/dynamic_message_unittest.cc:45:
google/protobuf/dynamic_message_unittest.cc:267:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(initial_space_used, message->SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/dynamic_message_unittest.cc:267:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(initial_space_used, message->SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/dynamic_message_unittest.cc:45:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
  CXX      google/protobuf/protobuf_test-extension_set_unittest.o
google/protobuf/extension_set_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::ExtensionSetTest_SerializationToArray_Test::TestBody()’:
google/protobuf/extension_set_unittest.cc:517:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = source.ByteSize();
                              ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/extension_set_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::ExtensionSetTest_SerializationToStream_Test::TestBody()’:
google/protobuf/extension_set_unittest.cc:538:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = source.ByteSize();
                              ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/extension_set_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::ExtensionSetTest_PackedSerializationToArray_Test::TestBody()’:
google/protobuf/extension_set_unittest.cc:561:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = source.ByteSize();
                              ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/extension_set_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::ExtensionSetTest_PackedSerializationToStream_Test::TestBody()’:
google/protobuf/extension_set_unittest.cc:582:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = source.ByteSize();
                              ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/extension_set_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::ExtensionSetTest_SpaceUsedExcludingSelf_Test::TestBody()’:
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:750:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(int32, 101);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:750:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(int32, 101);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:751:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(int64, 102);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:751:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(int64, 102);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:752:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(uint32, 103);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:752:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(uint32, 103);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:753:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(uint64, 104);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:753:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(uint64, 104);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:754:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sint32, 105);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:754:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sint32, 105);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:755:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sint64, 106);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:755:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sint64, 106);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:756:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(fixed32, 107);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:756:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(fixed32, 107);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:757:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(fixed64, 108);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:757:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(fixed64, 108);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:758:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sfixed32, 109);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:758:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sfixed32, 109);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:759:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sfixed64, 110);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:759:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(sfixed64, 110);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:760:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(float, 111);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:760:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(float, 111);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:761:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(double, 112);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:761:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(double, 112);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:742:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                               \
                                             ^
google/protobuf/extension_set_unittest.cc:762:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(bool, true);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:747:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:747:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());                       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:762:3: note: in expansion of macro ‘TEST_SCALAR_EXTENSIONS_SPACE_USED’
   TEST_SCALAR_EXTENSIONS_SPACE_USED(bool, true);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:766:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();
                                             ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:772:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:772:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());
     ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:778:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();
                                             ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:784:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:784:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());
     ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:789:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();
                                             ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:794:59: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     int min_expected_size = base_size + foreign.SpaceUsed();
                                                           ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:795:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:795:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());
     ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:838:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int32, int32, 101);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:838:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int32, int32, 101);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:838:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int32, int32, 101);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:838:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int32, int32, 101);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:839:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int64, int64, 102);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:839:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int64, int64, 102);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:839:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int64, int64, 102);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:839:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(int64, int64, 102);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:840:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint32, uint32, 103);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:840:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint32, uint32, 103);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:840:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint32, uint32, 103);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:840:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint32, uint32, 103);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:841:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint64, uint64, 104);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:841:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint64, uint64, 104);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:841:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint64, uint64, 104);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:841:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(uint64, uint64, 104);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:842:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint32, int32, 105);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:842:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint32, int32, 105);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:842:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint32, int32, 105);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:842:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint32, int32, 105);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:843:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint64, int64, 106);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:843:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint64, int64, 106);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:843:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint64, int64, 106);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:843:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sint64, int64, 106);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:844:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed32, uint32, 107);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:844:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed32, uint32, 107);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:844:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed32, uint32, 107);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:844:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed32, uint32, 107);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:845:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed64, uint64, 108);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:845:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed64, uint64, 108);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:845:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed64, uint64, 108);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:845:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(fixed64, uint64, 108);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:846:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed32, int32, 109);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:846:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed32, int32, 109);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:846:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed32, int32, 109);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:846:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed32, int32, 109);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:847:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed64, int64, 110);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:847:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed64, int64, 110);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:847:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed64, int64, 110);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:847:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(sfixed64, int64, 110);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:848:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(float, float, 111);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:848:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(float, float, 111);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:848:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(float, float, 111);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:848:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(float, float, 111);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:849:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(double, double, 112);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:849:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(double, double, 112);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:849:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(double, double, 112);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:849:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(double, double, 112);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:850:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(bool, bool, true);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:850:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(bool, bool, true);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:850:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(bool, bool, true);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:850:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(bool, bool, true);
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:811:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();                                \
                                             ^
google/protobuf/extension_set_unittest.cc:851:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(nested_enum, int,
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:815:61: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int empty_repeated_field_size = message.SpaceUsed();                \
                                                             ^
google/protobuf/extension_set_unittest.cc:851:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(nested_enum, int,
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:819:60: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
                                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:819:5: note: in expansion of macro ‘EXPECT_EQ’
     EXPECT_EQ(empty_repeated_field_size, message.SpaceUsed()) << #type;       \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:851:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(nested_enum, int,
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:835:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:835:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(expected_size, message.SpaceUsed()) << #type;                   \
     ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:851:3: note: in expansion of macro ‘TEST_REPEATED_EXTENSIONS_SPACE_USED’
   TEST_REPEATED_EXTENSIONS_SPACE_USED(nested_enum, int,
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:857:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();
                                             ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:868:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:868:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());
     ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:873:45: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     const int base_size = message.SpaceUsed();
                                             ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/extension_set_unittest.cc:883:70: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
         (16 - kMinRepeatedFieldAllocationSize) * prototype.SpaceUsed();
                                                                      ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/extension_set_unittest.cc:37:
google/protobuf/extension_set_unittest.cc:884:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
     EXPECT_LE(min_expected_size, message.SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2055:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/extension_set_unittest.cc:884:5: note: in expansion of macro ‘EXPECT_LE’
     EXPECT_LE(min_expected_size, message.SpaceUsed());
     ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/extension_set_unittest.cc:37:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
  CXX      google/protobuf/protobuf_test-generated_message_reflection_unittest.o
  CXX      google/protobuf/protobuf_test-map_field_test.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_field_test.cc:38:
./google/protobuf/map_test_util_impl.h: In instantiation of ‘static void google::protobuf::MapTestUtilImpl::ExpectMapFieldsSetInitialized(const MapMessage&) [with EnumType = protobuf_unittest::MapEnum; EnumType enum_value = (protobuf_unittest::MapEnum)0; MapMessage = protobuf_unittest::TestMap]’:
./google/protobuf/map_test_util.inc:123:14:   required from here
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:149:28: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   GTEST_ASSERT_(pred_format(#v1, #v2, v1, v2), \
                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/map_test_util_impl.h:413:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.map_int32_foreign_message().at(0).ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_unittest.pb.h:26,
                 from ./google/protobuf/map_test_util.h:34,
                 from google/protobuf/map_field_test.cc:38:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-map_test.o
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::MapFieldReflectionTest_RegularFields_Test::TestBody()’:
google/protobuf/map_test.cc:1058:69: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedPtrField<Message>(message, fd_map_int32_int32);
                                                                     ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1060:70: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedPtrField<Message>(message, fd_map_int32_double);
                                                                      ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1062:71: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedPtrField<Message>(message, fd_map_string_string);
                                                                       ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1064:79: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedPtrField<Message>(message, fd_map_int32_foreign_message);
                                                                               ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1068:74: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->MutableRepeatedPtrField<Message>(&message, fd_map_int32_int32);
                                                                          ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1070:75: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->MutableRepeatedPtrField<Message>(&message, fd_map_int32_double);
                                                                           ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1072:76: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->MutableRepeatedPtrField<Message>(&message, fd_map_string_string);
                                                                            ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:1075:74: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
                                              fd_map_int32_foreign_message);
                                                                          ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::GeneratedMapFieldTest_SerializationToArray_Test::TestBody()’:
google/protobuf/map_test.cc:2142:32: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message1.ByteSize();
                                ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::GeneratedMapFieldTest_SerializationToStream_Test::TestBody()’:
google/protobuf/map_test.cc:2155:32: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message1.ByteSize();
                                ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::GeneratedMapFieldTest_MissedValueTextFormat_Test::TestBody()’:
google/protobuf/map_test.cc:2372:34: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(11, message.ByteSize());
                                  ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2372:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(11, message.ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::GeneratedMapFieldReflectionTest_SpaceUsed_Test::TestBody()’:
google/protobuf/map_test.cc:2484:58: warning: ‘int google::protobuf::Reflection::SpaceUsed(const google::protobuf::Message&) const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(0, message.GetReflection()->SpaceUsed(message));
                                                          ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2484:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(0, message.GetReflection()->SpaceUsed(message));
   ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:433:7: note: declared here
   int SpaceUsed(const Message& message) const {
       ^~~~~~~~~
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::MapFieldInDynamicMessageTest_MapSpaceUsed_Test::TestBody()’:
google/protobuf/map_test.cc:2778:47: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   int initial_space_used = message->SpaceUsed();
                                               ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:2781:52: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(initial_space_used, message->SpaceUsed());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2781:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(initial_space_used, message->SpaceUsed());
   ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatForMapFieldTest_MapByteSize_Test::TestBody()’:
google/protobuf/map_test.cc:2955:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2955:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:2955:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2955:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:2955:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2955:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:2955:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2955:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:2955:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2955:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:2957:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:2957:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatForMapFieldTest_SerializeMap_Test::TestBody()’:
google/protobuf/map_test.cc:2970:22: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
     message.ByteSize();
                      ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatForMapFieldTest_MapByteSizeDynamicMessage_Test::TestBody()’:
google/protobuf/map_test.cc:3029:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3029:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3029:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3029:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3029:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3029:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3029:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3029:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3029:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3029:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), expected_size);
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc:3045:50: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int duplicate_size = dynamic_message->ByteSize();
                                                  ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3049:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3049:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3049:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3049:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3049:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3049:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3049:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3049:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
google/protobuf/map_test.cc:3049:39: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
                                       ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/map_test.cc:3049:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(dynamic_message->ByteSize(), duplicate_serialized_data.size());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc:3055:40: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = dynamic_message->ByteSize();
                                        ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::TextFormatMapTest_NoDisableReflectionIterator_Test::TestBody()’:
google/protobuf/map_test.cc:3355:71: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       reflection->MutableRepeatedPtrField<Message>(&source, field_desc);
                                                                       ^
In file included from ./google/protobuf/map_proto2_unittest.pb.h:31:0,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/map_test.cc:55:
./google/protobuf/map_test_util_impl.h: In instantiation of ‘static void google::protobuf::MapTestUtilImpl::ExpectMapFieldsSetInitialized(const MapMessage&) [with EnumType = protobuf_unittest::MapEnum; EnumType enum_value = (protobuf_unittest::MapEnum)0; MapMessage = protobuf_unittest::TestMap]’:
./google/protobuf/map_test_util.inc:123:14:   required from here
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:149:28: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   GTEST_ASSERT_(pred_format(#v1, #v2, v1, v2), \
                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/map_test_util_impl.h:413:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.map_int32_foreign_message().at(0).ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In instantiation of ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerialization(const T&) [with T = protobuf_unittest::TestMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/map_test.cc:3171:58:   required from here
google/protobuf/map_test.cc:3148:13: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   const int size = t.ByteSize();
             ^~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In instantiation of ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerialization(const T&) [with T = protobuf_unittest::TestSubmessageMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/map_test.cc:3242:5:   required from here
google/protobuf/map_test.cc:3148:13: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   const int size = t.ByteSize();
             ^~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In instantiation of ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerializationWithSerializeToCodedStream(const T&) [with T = protobuf_unittest::TestMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/map_test.cc:3158:3:   required from ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerialization(const T&) [with T = protobuf_unittest::TestMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’
google/protobuf/map_test.cc:3171:58:   required from here
google/protobuf/map_test.cc:3135:13: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   const int size = t.ByteSize();
             ^~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In instantiation of ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerializationWithSerializePartialToCodedStream(const T&) [with T = protobuf_unittest::TestMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/map_test.cc:3159:3:   required from ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerialization(const T&) [with T = protobuf_unittest::TestMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’
google/protobuf/map_test.cc:3171:58:   required from here
google/protobuf/map_test.cc:3121:13: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   const int size = t.ByteSize();
             ^~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In instantiation of ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerializationWithSerializeToCodedStream(const T&) [with T = protobuf_unittest::TestSubmessageMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/map_test.cc:3158:3:   required from ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerialization(const T&) [with T = protobuf_unittest::TestSubmessageMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’
google/protobuf/map_test.cc:3242:5:   required from here
google/protobuf/map_test.cc:3135:13: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   const int size = t.ByteSize();
             ^~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/map_test.cc: In instantiation of ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerializationWithSerializePartialToCodedStream(const T&) [with T = protobuf_unittest::TestSubmessageMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/map_test.cc:3159:3:   required from ‘std::__cxx11::string google::protobuf::internal::{anonymous}::DeterministicSerialization(const T&) [with T = protobuf_unittest::TestSubmessageMaps; std::__cxx11::string = std::__cxx11::basic_string<char>]’
google/protobuf/map_test.cc:3242:5:   required from here
google/protobuf/map_test.cc:3121:13: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   const int size = t.ByteSize();
             ^~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_proto2_unittest.pb.h:26,
                 from google/protobuf/map_test.cc:54:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-message_unittest.o
  CXX      google/protobuf/protobuf_test-no_field_presence_test.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from google/protobuf/no_field_presence_test.cc:37:
google/protobuf/no_field_presence_test.cc: In member function ‘virtual void google::protobuf::{anonymous}::NoFieldPresenceTest_OneofPresence_Test::TestBody()’:
google/protobuf/no_field_presence_test.cc:570:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/no_field_presence_test.cc:570:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from google/protobuf/no_field_presence_test.cc:33:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-preserve_unknown_enum_test.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from google/protobuf/preserve_unknown_enum_test.cc:36:
google/protobuf/preserve_unknown_enum_test.cc: In member function ‘virtual void google::protobuf::PreserveUnknownEnumTest_Proto2HidesUnknownValues_Test::TestBody()’:
google/protobuf/preserve_unknown_enum_test.cc:140:34: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message2.ByteSize());
                                  ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/preserve_unknown_enum_test.cc:140:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message2.ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from google/protobuf/preserve_unknown_enum_test.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from google/protobuf/preserve_unknown_enum_test.cc:36:
google/protobuf/preserve_unknown_enum_test.cc: In member function ‘virtual void google::protobuf::PreserveUnknownEnumTest_DynamicProto2HidesUnknownValues_Test::TestBody()’:
google/protobuf/preserve_unknown_enum_test.cc:168:34: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message2.ByteSize());
                                  ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/preserve_unknown_enum_test.cc:168:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message2.ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from google/protobuf/preserve_unknown_enum_test.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-proto3_arena_lite_unittest.o
  CXX      google/protobuf/protobuf_test-proto3_arena_unittest.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/proto3_arena_unittest.cc:35:
google/protobuf/proto3_arena_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::Proto3ArenaTest_UnknownFields_Test::TestBody()’:
google/protobuf/proto3_arena_unittest.cc:140:31: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   ASSERT_NE(original.ByteSize(), arena_message->ByteSize());
                               ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:168:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_FATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2068:3: note: in expansion of macro ‘ASSERT_PRED_FORMAT2’
   ASSERT_PRED_FORMAT2(::testing::internal::CmpHelperNE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2086:32: note: in expansion of macro ‘GTEST_ASSERT_NE’
 # define ASSERT_NE(val1, val2) GTEST_ASSERT_NE(val1, val2)
                                ^~~~~~~~~~~~~~~
google/protobuf/proto3_arena_unittest.cc:140:3: note: in expansion of macro ‘ASSERT_NE’
   ASSERT_NE(original.ByteSize(), arena_message->ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/proto3_arena_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/proto3_arena_unittest.cc:35:
google/protobuf/proto3_arena_unittest.cc:140:58: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   ASSERT_NE(original.ByteSize(), arena_message->ByteSize());
                                                          ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:168:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_FATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2068:3: note: in expansion of macro ‘ASSERT_PRED_FORMAT2’
   ASSERT_PRED_FORMAT2(::testing::internal::CmpHelperNE, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2086:32: note: in expansion of macro ‘GTEST_ASSERT_NE’
 # define ASSERT_NE(val1, val2) GTEST_ASSERT_NE(val1, val2)
                                ^~~~~~~~~~~~~~~
google/protobuf/proto3_arena_unittest.cc:140:3: note: in expansion of macro ‘ASSERT_NE’
   ASSERT_NE(original.ByteSize(), arena_message->ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest.pb.h:26,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/proto3_arena_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-proto3_lite_unittest.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from ./google/protobuf/proto3_lite_unittest.inc:36,
                 from google/protobuf/proto3_lite_unittest.cc:37:
./google/protobuf/proto3_lite_unittest.inc: In member function ‘virtual void google::protobuf::{anonymous}::Proto3LiteTest_Swap_Test::TestBody()’:
./google/protobuf/proto3_lite_unittest.inc:130:27: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
                           ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
./google/protobuf/proto3_lite_unittest.inc:130:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_proto3.pb.h:26,
                 from google/protobuf/proto3_lite_unittest.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from ./google/protobuf/proto3_lite_unittest.inc:36,
                 from google/protobuf/proto3_lite_unittest.cc:37:
./google/protobuf/proto3_lite_unittest.inc:130:27: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
                           ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
./google/protobuf/proto3_lite_unittest.inc:130:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_proto3.pb.h:26,
                 from google/protobuf/proto3_lite_unittest.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from ./google/protobuf/proto3_lite_unittest.inc:36,
                 from google/protobuf/proto3_lite_unittest.cc:37:
./google/protobuf/proto3_lite_unittest.inc:130:27: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
                           ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
./google/protobuf/proto3_lite_unittest.inc:130:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_proto3.pb.h:26,
                 from google/protobuf/proto3_lite_unittest.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from ./google/protobuf/proto3_lite_unittest.inc:36,
                 from google/protobuf/proto3_lite_unittest.cc:37:
./google/protobuf/proto3_lite_unittest.inc:130:27: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
                           ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
./google/protobuf/proto3_lite_unittest.inc:130:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_proto3.pb.h:26,
                 from google/protobuf/proto3_lite_unittest.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from ./google/protobuf/proto3_lite_unittest.inc:36,
                 from google/protobuf/proto3_lite_unittest.cc:37:
./google/protobuf/proto3_lite_unittest.inc:130:27: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
                           ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/proto3_lite_unittest.inc:130:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_proto3.pb.h:26,
                 from google/protobuf/proto3_lite_unittest.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from ./google/protobuf/proto3_lite_unittest.inc:36,
                 from google/protobuf/proto3_lite_unittest.cc:37:
./google/protobuf/proto3_lite_unittest.inc:130:44: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
                                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/proto3_lite_unittest.inc:130:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(msg1.ByteSize(), msg2.ByteSize() + 1);
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_proto3.pb.h:26,
                 from google/protobuf/proto3_lite_unittest.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-reflection_ops_unittest.o
  CXX      google/protobuf/protobuf_test-repeated_field_reflection_unittest.o
google/protobuf/repeated_field_reflection_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::RepeatedFieldReflectionTest_RegularFields_Test::TestBody()’:
google/protobuf/repeated_field_reflection_unittest.cc:81:63: warning: ‘const google::protobuf::RepeatedField<Element>& google::protobuf::Reflection::GetRepeatedField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = int]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedField<int32>(message, fd_repeated_int32);
                                                               ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:771:27: note: declared here
   const RepeatedField<T>& GetRepeatedField(const Message& msg,
                           ^~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:83:65: warning: ‘const google::protobuf::RepeatedField<Element>& google::protobuf::Reflection::GetRepeatedField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = double]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedField<double>(message, fd_repeated_double);
                                                                 ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:771:27: note: declared here
   const RepeatedField<T>& GetRepeatedField(const Message& msg,
                           ^~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:87:68: warning: ‘google::protobuf::RepeatedField<Element>* google::protobuf::Reflection::MutableRepeatedField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = int]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->MutableRepeatedField<int32>(&message, fd_repeated_int32);
                                                                    ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:781:21: note: declared here
   RepeatedField<T>* MutableRepeatedField(Message* msg,
                     ^~~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:89:70: warning: ‘google::protobuf::RepeatedField<Element>* google::protobuf::Reflection::MutableRepeatedField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = double]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->MutableRepeatedField<double>(&message, fd_repeated_double);
                                                                      ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:781:21: note: declared here
   RepeatedField<T>* MutableRepeatedField(Message* msg,
                     ^~~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:93:73: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = std::__cxx11::basic_string<char>]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedPtrField<std::string>(message, fd_repeated_string);
                                                                         ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:96:76: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = protobuf_unittest::ForeignMessage]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
                                                 fd_repeated_foreign_message);
                                                                            ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:98:78: warning: ‘const google::protobuf::RepeatedPtrField<Element>& google::protobuf::Reflection::GetRepeatedPtrField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->GetRepeatedPtrField<Message>(message, fd_repeated_foreign_message);
                                                                              ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:792:30: note: declared here
   const RepeatedPtrField<T>& GetRepeatedPtrField(
                              ^~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:102:78: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = std::__cxx11::basic_string<char>]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       refl->MutableRepeatedPtrField<std::string>(&message, fd_repeated_string);
                                                                              ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:105:48: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = protobuf_unittest::ForeignMessage]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
           &message, fd_repeated_foreign_message);
                                                ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:108:73: warning: ‘google::protobuf::RepeatedPtrField<Element>* google::protobuf::Reflection::MutableRepeatedPtrField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = google::protobuf::Message]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
                                              fd_repeated_foreign_message);
                                                                         ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:803:24: note: declared here
   RepeatedPtrField<T>* MutableRepeatedPtrField(Message* msg,
                        ^~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::RepeatedFieldReflectionTest_ExtensionFields_Test::TestBody()’:
google/protobuf/repeated_field_reflection_unittest.cc:172:64: warning: ‘const google::protobuf::RepeatedField<Element>& google::protobuf::Reflection::GetRepeatedField(const google::protobuf::Message&, const google::protobuf::FieldDescriptor*) const [with T = long int]’ is deprecated: Please use GetRepeatedFieldRef() instead [-Wdeprecated-declarations]
                                     fd_repeated_int64_extension);
                                                                ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:771:27: note: declared here
   const RepeatedField<T>& GetRepeatedField(const Message& msg,
                           ^~~~~~~~~~~~~~~~
google/protobuf/repeated_field_reflection_unittest.cc:175:53: warning: ‘google::protobuf::RepeatedField<Element>* google::protobuf::Reflection::MutableRepeatedField(google::protobuf::Message*, const google::protobuf::FieldDescriptor*) const [with T = long int]’ is deprecated: Please use GetMutableRepeatedFieldRef() instead [-Wdeprecated-declarations]
       &extended_message, fd_repeated_int64_extension);
                                                     ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/repeated_field_reflection_unittest.cc:38:
./google/protobuf/message.h:781:21: note: declared here
   RepeatedField<T>* MutableRepeatedField(Message* msg,
                     ^~~~~~~~~~~~~~~~~~~~
  CXX      google/protobuf/protobuf_test-repeated_field_unittest.o
  CXX      google/protobuf/protobuf_test-text_format_unittest.o
google/protobuf/text_format_unittest.cc: In member function ‘virtual void google::protobuf::text_format_unittest::TextFormatTest_DefaultCustomFieldPrinter_Test::TestBody()’:
google/protobuf/text_format_unittest.cc:466:74: warning: ‘void google::protobuf::TextFormat::Printer::SetDefaultFieldValuePrinter(const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
   printer.SetDefaultFieldValuePrinter(new CustomUInt32FieldValuePrinter());
                                                                          ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:303:10: note: declared here
     void SetDefaultFieldValuePrinter(const FieldValuePrinter* printer);
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:60:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/text_format_unittest.cc:49:
google/protobuf/text_format_unittest.cc: In member function ‘virtual void google::protobuf::text_format_unittest::TextFormatTest_FieldSpecificCustomPrinter_Test::TestBody()’:
google/protobuf/text_format_unittest.cc:490:41: warning: ‘bool google::protobuf::TextFormat::Printer::RegisterFieldValuePrinter(const google::protobuf::FieldDescriptor*, const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
       new CustomInt32FieldValuePrinter()));
                                         ^
./../third_party/googletest/googletest/include/gtest/internal/gtest-internal.h:1378:34: note: in definition of macro ‘GTEST_TEST_BOOLEAN_’
       ::testing::AssertionResult(expression)) \
                                  ^~~~~~~~~~
google/protobuf/text_format_unittest.cc:488:3: note: in expansion of macro ‘EXPECT_TRUE’
   EXPECT_TRUE(printer.RegisterFieldValuePrinter(
   ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:357:10: note: declared here
     bool RegisterFieldValuePrinter(const FieldDescriptor* field,
          ^~~~~~~~~~~~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:60:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/text_format_unittest.cc:49:
google/protobuf/text_format_unittest.cc: In member function ‘virtual void google::protobuf::text_format_unittest::TextFormatTest_FieldSpecificCustomPrinterRegisterSameFieldTwice_Test::TestBody()’:
google/protobuf/text_format_unittest.cc:502:48: warning: ‘bool google::protobuf::TextFormat::Printer::RegisterFieldValuePrinter(const google::protobuf::FieldDescriptor*, const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
       field, new CustomInt32FieldValuePrinter()));
                                                ^
./../third_party/googletest/googletest/include/gtest/internal/gtest-internal.h:1378:34: note: in definition of macro ‘GTEST_TEST_BOOLEAN_’
       ::testing::AssertionResult(expression)) \
                                  ^~~~~~~~~~
google/protobuf/text_format_unittest.cc:501:3: note: in expansion of macro ‘ASSERT_TRUE’
   ASSERT_TRUE(printer.RegisterFieldValuePrinter(
   ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:357:10: note: declared here
     bool RegisterFieldValuePrinter(const FieldDescriptor* field,
          ^~~~~~~~~~~~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:60:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/text_format_unittest.cc:49:
google/protobuf/text_format_unittest.cc:505:65: warning: ‘bool google::protobuf::TextFormat::Printer::RegisterFieldValuePrinter(const google::protobuf::FieldDescriptor*, const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
   ASSERT_FALSE(printer.RegisterFieldValuePrinter(field, rejected));
                                                                 ^
./../third_party/googletest/googletest/include/gtest/internal/gtest-internal.h:1378:34: note: in definition of macro ‘GTEST_TEST_BOOLEAN_’
       ::testing::AssertionResult(expression)) \
                                  ^~~~~~~~~~
google/protobuf/text_format_unittest.cc:505:3: note: in expansion of macro ‘ASSERT_FALSE’
   ASSERT_FALSE(printer.RegisterFieldValuePrinter(field, rejected));
   ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:357:10: note: declared here
     bool RegisterFieldValuePrinter(const FieldDescriptor* field,
          ^~~~~~~~~~~~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:60:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/text_format_unittest.cc:49:
google/protobuf/text_format_unittest.cc: In member function ‘virtual void google::protobuf::text_format_unittest::TextFormatTest_ErrorCasesRegisteringFieldValuePrinterShouldFail_Test::TestBody()’:
google/protobuf/text_format_unittest.cc:515:65: warning: ‘bool google::protobuf::TextFormat::Printer::RegisterFieldValuePrinter(const google::protobuf::FieldDescriptor*, const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
       static_cast<const TextFormat::FieldValuePrinter*>(nullptr)));
                                                                 ^
./../third_party/googletest/googletest/include/gtest/internal/gtest-internal.h:1378:34: note: in definition of macro ‘GTEST_TEST_BOOLEAN_’
       ::testing::AssertionResult(expression)) \
                                  ^~~~~~~~~~
google/protobuf/text_format_unittest.cc:513:3: note: in expansion of macro ‘EXPECT_FALSE’
   EXPECT_FALSE(printer.RegisterFieldValuePrinter(
   ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:357:10: note: declared here
     bool RegisterFieldValuePrinter(const FieldDescriptor* field,
          ^~~~~~~~~~~~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:60:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/text_format_unittest.cc:49:
google/protobuf/text_format_unittest.cc:522:76: warning: ‘bool google::protobuf::TextFormat::Printer::RegisterFieldValuePrinter(const google::protobuf::FieldDescriptor*, const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
   EXPECT_FALSE(printer.RegisterFieldValuePrinter(nullptr, &my_field_printer));
                                                                            ^
./../third_party/googletest/googletest/include/gtest/internal/gtest-internal.h:1378:34: note: in definition of macro ‘GTEST_TEST_BOOLEAN_’
       ::testing::AssertionResult(expression)) \
                                  ^~~~~~~~~~
google/protobuf/text_format_unittest.cc:522:3: note: in expansion of macro ‘EXPECT_FALSE’
   EXPECT_FALSE(printer.RegisterFieldValuePrinter(nullptr, &my_field_printer));
   ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:357:10: note: declared here
     bool RegisterFieldValuePrinter(const FieldDescriptor* field,
          ^~~~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/text_format_unittest.cc: In member function ‘virtual void google::protobuf::text_format_unittest::TextFormatTest_CustomPrinterForComments_Test::TestBody()’:
google/protobuf/text_format_unittest.cc:553:75: warning: ‘void google::protobuf::TextFormat::Printer::SetDefaultFieldValuePrinter(const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
   printer.SetDefaultFieldValuePrinter(new CustomMessageFieldValuePrinter());
                                                                           ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:303:10: note: declared here
     void SetDefaultFieldValuePrinter(const FieldValuePrinter* printer);
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~
google/protobuf/text_format_unittest.cc: In member function ‘virtual void google::protobuf::text_format_unittest::TextFormatTest_CustomPrinterForMultilineComments_Test::TestBody()’:
google/protobuf/text_format_unittest.cc:638:74: warning: ‘void google::protobuf::TextFormat::Printer::SetDefaultFieldValuePrinter(const google::protobuf::TextFormat::FieldValuePrinter*)’ is deprecated: Please use FastFieldValuePrinter [-Wdeprecated-declarations]
   printer.SetDefaultFieldValuePrinter(new CustomMultilineCommentPrinter());
                                                                          ^
In file included from google/protobuf/text_format_unittest.cc:35:0:
./google/protobuf/text_format.h:303:10: note: declared here
     void SetDefaultFieldValuePrinter(const FieldValuePrinter* printer);
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~
  CXX      google/protobuf/protobuf_test-unknown_field_set_unittest.o
google/protobuf/unknown_field_set_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::UnknownFieldSetTest_SpaceUsed_Test::TestBody()’:
google/protobuf/unknown_field_set_unittest.cc:539:43: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   int base_size = empty_message.SpaceUsed();
                                           ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
google/protobuf/unknown_field_set_unittest.cc:541:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(base_size, empty_message.SpaceUsed());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:541:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(base_size, empty_message.SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
google/protobuf/unknown_field_set_unittest.cc:545:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(base_size, empty_message.SpaceUsed());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:545:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(base_size, empty_message.SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:546:39: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   base_size = empty_message.SpaceUsed();
                                       ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
google/protobuf/unknown_field_set_unittest.cc:549:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(base_size, empty_message.SpaceUsed());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:549:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(base_size, empty_message.SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:550:39: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   base_size = empty_message.SpaceUsed();
                                       ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
google/protobuf/unknown_field_set_unittest.cc:553:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(base_size, empty_message.SpaceUsed());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:553:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(base_size, empty_message.SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:554:39: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   base_size = empty_message.SpaceUsed();
                                       ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
google/protobuf/unknown_field_set_unittest.cc:557:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(base_size, empty_message.SpaceUsed());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:557:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(base_size, empty_message.SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:558:39: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   base_size = empty_message.SpaceUsed();
                                       ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
google/protobuf/unknown_field_set_unittest.cc:561:48: warning: ‘int google::protobuf::Message::SpaceUsed() const’ is deprecated: Please use SpaceUsedLong() instead [-Wdeprecated-declarations]
   EXPECT_LT(base_size, empty_message.SpaceUsed());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2057:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal::CmpHelperLT, val1, val2)
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/unknown_field_set_unittest.cc:561:3: note: in expansion of macro ‘EXPECT_LT’
   EXPECT_LT(base_size, empty_message.SpaceUsed());
   ^
In file included from ./google/protobuf/unittest.pb.h:31:0,
                 from ./google/protobuf/test_util.h:38,
                 from google/protobuf/unknown_field_set_unittest.cc:43:
./google/protobuf/message.h:296:7: note: declared here
   int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }
       ^~~~~~~~~
  CXX      google/protobuf/protobuf_test-well_known_types_unittest.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::WellKnownTypesTest_AllKnownTypesAreIncluded_Test::TestBody()’:
google/protobuf/well_known_types_unittest.cc:45:45: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.any_field().ByteSize());
                                             ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:45:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.any_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:46:45: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.api_field().ByteSize());
                                             ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:46:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.api_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:47:50: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.duration_field().ByteSize());
                                                  ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:47:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.duration_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:48:47: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.empty_field().ByteSize());
                                               ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:48:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.empty_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:49:52: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.field_mask_field().ByteSize());
                                                    ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:49:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.field_mask_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:50:56: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.source_context_field().ByteSize());
                                                        ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:50:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.source_context_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:51:48: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.struct_field().ByteSize());
                                                ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:51:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.struct_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:52:51: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.timestamp_field().ByteSize());
                                                   ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:52:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.timestamp_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:53:46: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.type_field().ByteSize());
                                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:53:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.type_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./../third_party/googletest/googlemock/include/gmock/internal/gmock-internal-utils.h:47,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock-actions.h:51,
                 from ./../third_party/googletest/googlemock/include/gmock/gmock.h:59,
                 from ./google/protobuf/testing/googletest.h:41,
                 from google/protobuf/well_known_types_unittest.cc:33:
google/protobuf/well_known_types_unittest.cc:54:47: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.int32_field().ByteSize());
                                               ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/well_known_types_unittest.cc:54:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.int32_field().ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_well_known_types.pb.h:26,
                 from google/protobuf/well_known_types_unittest.cc:30:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_test-wire_format_unittest.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_ByteSize_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:227:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:227:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:227:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:227:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:227:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:227:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:227:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:227:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:227:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:227:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:229:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:229:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_ByteSizeExtensions_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:237:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:237:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:237:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:237:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:237:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:237:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:237:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:237:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:237:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:237:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:239:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:239:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_ByteSizePacked_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:247:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:247:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:247:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:247:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:247:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:247:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:247:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:247:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:247:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:247:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:249:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:249:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_ByteSizePackedExtensions_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:257:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:257:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:257:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:257:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:257:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:257:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:257:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:257:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:257:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:257:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:259:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:259:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_ByteSizeOneof_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:267:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:267:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:267:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:267:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:267:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:267:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:267:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2050:32: note: in expansion of macro ‘GTEST_IS_NULL_LITERAL_’
                       EqHelper<GTEST_IS_NULL_LITERAL_(val1)>::Compare, \
                                ^~~~~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:267:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:267:30: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
                              ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:267:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(message.ByteSize(), WireFormat::ByteSize(message));
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/test_util.inc:44,
                 from ./google/protobuf/test_util.h:43,
                 from google/protobuf/wire_format_unittest.cc:40:
google/protobuf/wire_format_unittest.cc:270:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/wire_format_unittest.cc:270:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_Serialize_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:280:31: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message.ByteSize();
                               ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_SerializeExtensions_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:310:31: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message.ByteSize();
                               ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_SerializeFieldsAndExtensions_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:340:31: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message.ByteSize();
                               ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_SerializeOneof_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:374:31: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message.ByteSize();
                               ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_SerializeMessageSetVariousWaysAreEqual_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:485:35: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   int size = message_set.ByteSize();
                                   ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In member function ‘virtual void google::protobuf::internal::{anonymous}::WireFormatTest_ParseMessageSetWithReverseTagOrder_Test::TestBody()’:
google/protobuf/wire_format_unittest.cc:598:49: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
     coded_output.WriteVarint32(message.ByteSize());
                                                 ^
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In instantiation of ‘void google::protobuf::internal::{anonymous}::Proto3PrimitiveRepeatedWireFormatTest::TestSerialization(Proto*, const string&) [with Proto = proto3_arena_unittest::TestAllTypes; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/wire_format_unittest.cc:1038:57:   required from here
google/protobuf/wire_format_unittest.cc:992:9: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
     int size = message->ByteSize();
         ^~~~
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/wire_format_unittest.cc: In instantiation of ‘void google::protobuf::internal::{anonymous}::Proto3PrimitiveRepeatedWireFormatTest::TestSerialization(Proto*, const string&) [with Proto = proto3_arena_unittest::TestUnpackedTypes; std::__cxx11::string = std::__cxx11::basic_string<char>]’:
google/protobuf/wire_format_unittest.cc:1041:66:   required from here
google/protobuf/wire_format_unittest.cc:992:9: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
     int size = message->ByteSize();
         ^~~~
In file included from ./google/protobuf/implicit_weak_message.h:38:0,
                 from ./google/protobuf/parse_context.h:42,
                 from ./google/protobuf/wire_format.h:45,
                 from google/protobuf/wire_format_unittest.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/io/protobuf_test-coded_stream_unittest.o
google/protobuf/io/coded_stream_unittest.cc: In member function ‘virtual void google::protobuf::io::{anonymous}::CodedStreamTest_ReadStringNoReservationSizeIsOverTheTotalBytesLimit_Test::TestBody()’:
google/protobuf/io/coded_stream_unittest.cc:869:42: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
     coded_input.SetTotalBytesLimit(16, 16);
                                          ^
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
google/protobuf/io/coded_stream_unittest.cc: In member function ‘virtual void google::protobuf::io::{anonymous}::CodedStreamTest_ReadStringNoReservationSizeIsOverTheClosestLimit_GlobalLimitIsCloser_Test::TestBody()’:
google/protobuf/io/coded_stream_unittest.cc:891:42: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
     coded_input.SetTotalBytesLimit(16, 16);
                                          ^
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
google/protobuf/io/coded_stream_unittest.cc: In member function ‘virtual void google::protobuf::io::{anonymous}::CodedStreamTest_ReadStringNoReservationSizeIsOverTheClosestLimit_LocalLimitIsCloser_Test::TestBody()’:
google/protobuf/io/coded_stream_unittest.cc:913:68: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
     coded_input.SetTotalBytesLimit(sizeof(buffer_), sizeof(buffer_));
                                                                    ^
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
google/protobuf/io/coded_stream_unittest.cc: In member function ‘virtual void google::protobuf::io::{anonymous}::CodedStreamTest_TotalBytesLimit_Test::TestBody()’:
google/protobuf/io/coded_stream_unittest.cc:1185:40: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
   coded_input.SetTotalBytesLimit(16, -1);
                                        ^
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
google/protobuf/io/coded_stream_unittest.cc:1205:40: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
   coded_input.SetTotalBytesLimit(32, -1);
                                        ^
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
google/protobuf/io/coded_stream_unittest.cc: In member function ‘virtual void google::protobuf::io::{anonymous}::CodedStreamTest_TotalBytesLimitNotValidMessageEnd_Test::TestBody()’:
google/protobuf/io/coded_stream_unittest.cc:1218:40: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
   coded_input.SetTotalBytesLimit(16, -1);
                                        ^
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
google/protobuf/io/coded_stream_unittest.cc: In instantiation of ‘void google::protobuf::io::{anonymous}::CodedStreamTest_ReadStringReservesMemoryOnTotalLimit_DD::DoSingleCase(const CaseType&) [with CaseType = int]’:
google/protobuf/io/coded_stream_unittest.cc:740:1:   required from here
google/protobuf/io/coded_stream_unittest.cc:746:35: warning: ‘void google::protobuf::io::CodedInputStream::SetTotalBytesLimit(int, int)’ is deprecated: Please use the single parameter version of SetTotalBytesLimit(). The second parameter is ignored. [-Wdeprecated-declarations]
     coded_input.SetTotalBytesLimit(sizeof(kRawBytes), sizeof(kRawBytes));
     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
In file included from google/protobuf/io/coded_stream_unittest.cc:37:0:
./google/protobuf/io/coded_stream.h:397:8: note: declared here
   void SetTotalBytesLimit(int total_bytes_limit, int) {
        ^~~~~~~~~~~~~~~~~~
  CXX      google/protobuf/io/protobuf_test-printer_unittest.o
  CXX      google/protobuf/io/protobuf_test-tokenizer_unittest.o
  CXX      google/protobuf/io/protobuf_test-zero_copy_stream_unittest.o
  CXX      google/protobuf/compiler/protobuf_test-annotation_test_util.o
  CXX      google/protobuf/compiler/protobuf_test-importer_unittest.o
  CXX      google/protobuf/compiler/protobuf_test-mock_code_generator.o
  CXX      google/protobuf/compiler/protobuf_test-parser_unittest.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-cpp_bootstrap_unittest.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-cpp_move_unittest.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-cpp_unittest.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-cpp_plugin_unittest.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-metadata_test.o
  CXX      google/protobuf/compiler/java/protobuf_test-java_plugin_unittest.o
  CXX      google/protobuf/compiler/java/protobuf_test-java_doc_comment_unittest.o
  CXX      google/protobuf/compiler/objectivec/protobuf_test-objectivec_helpers_unittest.o
  CXX      google/protobuf/compiler/python/protobuf_test-python_plugin_unittest.o
  CXX      google/protobuf/compiler/ruby/protobuf_test-ruby_generator_unittest.o
  CXX      google/protobuf/compiler/csharp/protobuf_test-csharp_bootstrap_unittest.o
  CXX      google/protobuf/compiler/csharp/protobuf_test-csharp_generator_unittest.o
  CXX      google/protobuf/util/protobuf_test-delimited_message_util_test.o
  CXX      google/protobuf/util/protobuf_test-field_comparator_test.o
  CXX      google/protobuf/util/protobuf_test-field_mask_util_test.o
  CXX      google/protobuf/util/internal/protobuf_test-default_value_objectwriter_test.o
  CXX      google/protobuf/util/internal/protobuf_test-json_objectwriter_test.o
  CXX      google/protobuf/util/internal/protobuf_test-json_stream_parser_test.o
  CXX      google/protobuf/util/internal/protobuf_test-protostream_objectsource_test.o
  CXX      google/protobuf/util/internal/protobuf_test-protostream_objectwriter_test.o
  CXX      google/protobuf/util/internal/protobuf_test-type_info_test_helper.o
  CXX      google/protobuf/util/protobuf_test-json_util_test.o
  CXX      google/protobuf/util/protobuf_test-message_differencer_unittest.o
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSmartListTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1091:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_a(1);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1092:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_a(2);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1093:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_a(3);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1094:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_a(4);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1095:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_a(5);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1096:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg2.add_rm()->set_a(2);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1097:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg2.add_rm()->set_a(6);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1098:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg2.add_rm()->set_a(4);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSmartSetTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1140:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem1_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1141:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem2_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1142:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem3_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1144:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem3_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1145:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem1_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1146:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem2_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSmartSetTest_IdenticalElements_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1171:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1172:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1173:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1174:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSmartSetTest_PreviouslyMatch_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1203:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem1_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1204:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem2_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1205:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem1_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1206:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem2_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSmartSet_MultipleMatches_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1246:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem1_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1247:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem2_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1248:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem3_1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1249:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem2_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1250:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem3_2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSmartSet_MultipleMatchesNoReporter_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1274:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem1;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1275:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1276:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg1.add_rm() = elem3;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1277:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem2;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1278:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem3;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1279:16: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   *msg2.add_rm() = elem4;
                ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSetTest_PartialSimple_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1501:12: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   a.add_rm()->set_c(1);
            ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1502:12: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   a.add_rm()->set_c(0);
            ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1507:12: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   b.add_rm()->set_c(1);
            ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1508:12: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   b.add_rm();
            ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1513:12: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   c.add_rm();
            ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1514:12: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   c.add_rm()->set_c(1);
            ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_RepeatedFieldSetTest_Partial_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1529:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_a(1);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1530:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_b(2);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1531:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_c(3);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1537:53: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   protobuf_unittest::TestField* field = msg2.add_rm();
                                                     ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1540:23: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   field = msg2.add_rm();
                       ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1543:23: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   field = msg2.add_rm();
                       ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_IgnoreField_Message_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:1977:23: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   field = msg1.add_rm();
                       ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:1980:23: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   field = msg2.add_rm();
                       ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_IgnoreField_NestedMessage_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:2065:23: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   field = msg1.add_rm();
                       ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:2069:23: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   field = msg2.add_rm();
                       ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::MessageDifferencerTest_FieldContextParentFieldsTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:2298:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg1.add_rm()->set_c(1);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:2300:15: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   msg2.add_rm()->set_c(1);
               ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::ComparisonTest_IgnoredNoChangeTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:2944:22: warning: ‘void protobuf_unittest::TestDiffMessage::set_v(google::protobuf::int32)’ is deprecated [-Wdeprecated-declarations]
   proto1diff_.set_v(3);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1543:13: note: declared here
 inline void TestDiffMessage::set_v(::PROTOBUF_NAMESPACE_ID::int32 value) {
             ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:2945:22: warning: ‘void protobuf_unittest::TestDiffMessage::set_v(google::protobuf::int32)’ is deprecated [-Wdeprecated-declarations]
   proto2diff_.set_v(3);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1543:13: note: declared here
 inline void TestDiffMessage::set_v(::PROTOBUF_NAMESPACE_ID::int32 value) {
             ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::ComparisonTest_IgnoredAddTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:2957:22: warning: ‘void protobuf_unittest::TestDiffMessage::set_v(google::protobuf::int32)’ is deprecated [-Wdeprecated-declarations]
   proto2diff_.set_v(3);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1543:13: note: declared here
 inline void TestDiffMessage::set_v(::PROTOBUF_NAMESPACE_ID::int32 value) {
             ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::ComparisonTest_IgnoredDeleteTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:2969:22: warning: ‘void protobuf_unittest::TestDiffMessage::set_v(google::protobuf::int32)’ is deprecated [-Wdeprecated-declarations]
   proto1diff_.set_v(3);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1543:13: note: declared here
 inline void TestDiffMessage::set_v(::PROTOBUF_NAMESPACE_ID::int32 value) {
             ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::ComparisonTest_IgnoredModifyTest_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:2981:22: warning: ‘void protobuf_unittest::TestDiffMessage::set_v(google::protobuf::int32)’ is deprecated [-Wdeprecated-declarations]
   proto1diff_.set_v(3);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1543:13: note: declared here
 inline void TestDiffMessage::set_v(::PROTOBUF_NAMESPACE_ID::int32 value) {
             ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:2982:22: warning: ‘void protobuf_unittest::TestDiffMessage::set_v(google::protobuf::int32)’ is deprecated [-Wdeprecated-declarations]
   proto2diff_.set_v(4);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1543:13: note: declared here
 inline void TestDiffMessage::set_v(::PROTOBUF_NAMESPACE_ID::int32 value) {
             ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc: In member function ‘virtual void google::protobuf::{anonymous}::ComparisonTest_IgnoredRepeatedNested_Test::TestBody()’:
google/protobuf/util/message_differencer_unittest.cc:3075:22: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   proto1diff_.add_rm()->set_c(0);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:3076:22: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   proto1diff_.add_rm()->set_c(1);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:3077:22: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   proto2diff_.add_rm()->set_c(2);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
google/protobuf/util/message_differencer_unittest.cc:3078:22: warning: ‘protobuf_unittest::TestField* protobuf_unittest::TestDiffMessage::add_rm()’ is deprecated [-Wdeprecated-declarations]
   proto2diff_.add_rm()->set_c(3);
                      ^
In file included from google/protobuf/util/message_differencer_unittest.cc:53:0:
./google/protobuf/util/message_differencer_unittest.pb.h:1872:40: note: declared here
 inline ::protobuf_unittest::TestField* TestDiffMessage::add_rm() {
                                        ^~~~~~~~~~~~~~~
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from ./google/protobuf/map_test_util.inc:31,
                 from ./google/protobuf/map_test_util.h:38,
                 from google/protobuf/util/message_differencer_unittest.cc:45:
./google/protobuf/map_test_util_impl.h: In instantiation of ‘static void google::protobuf::MapTestUtilImpl::ExpectMapFieldsSetInitialized(const MapMessage&) [with EnumType = protobuf_unittest::MapEnum; EnumType enum_value = (protobuf_unittest::MapEnum)0; MapMessage = protobuf_unittest::TestMap]’:
./google/protobuf/map_test_util.inc:123:14:   required from here
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:149:28: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   GTEST_ASSERT_(pred_format(#v1, #v2, v1, v2), \
                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/map_test_util_impl.h:413:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.map_int32_foreign_message().at(0).ByteSize());
   ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/any_test.pb.h:26,
                 from google/protobuf/util/message_differencer_unittest.cc:44:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/util/protobuf_test-time_util_test.o
  CXX      google/protobuf/util/protobuf_test-type_resolver_util_test.o
  CXX      google/protobuf/compiler/protobuf_test-command_line_interface_unittest.o
  CXX      google/protobuf/protobuf_test-arena_test_util.o
  CXX      google/protobuf/protobuf_test-test_util.o
  CXX      google/protobuf/testing/protobuf_test-googletest.o
  CXX      google/protobuf/testing/protobuf_test-file.o
  CXX      google/protobuf/protobuf_test-map_lite_unittest.pb.o
  CXX      google/protobuf/protobuf_test-unittest_lite.pb.o
  CXX      google/protobuf/protobuf_test-unittest_no_arena_lite.pb.o
  CXX      google/protobuf/protobuf_test-unittest_import_lite.pb.o
  CXX      google/protobuf/protobuf_test-unittest_import_public_lite.pb.o
  CXX      google/protobuf/protobuf_test-any_test.pb.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-cpp_test_bad_identifiers.pb.o
  CXX      google/protobuf/compiler/cpp/protobuf_test-cpp_test_large_enum_value.pb.o
  CXX      google/protobuf/protobuf_test-map_proto2_unittest.pb.o
  CXX      google/protobuf/protobuf_test-map_unittest.pb.o
  CXX      google/protobuf/protobuf_test-unittest_arena.pb.o
  CXX      google/protobuf/protobuf_test-unittest_custom_options.pb.o
  CXX      google/protobuf/protobuf_test-unittest_drop_unknown_fields.pb.o
  CXX      google/protobuf/protobuf_test-unittest_embed_optimize_for.pb.o
  CXX      google/protobuf/protobuf_test-unittest_empty.pb.o
  CXX      google/protobuf/protobuf_test-unittest_enormous_descriptor.pb.o
  CXX      google/protobuf/protobuf_test-unittest_import.pb.o
  CXX      google/protobuf/protobuf_test-unittest_import_public.pb.o
  CXX      google/protobuf/protobuf_test-unittest_lazy_dependencies.pb.o
  CXX      google/protobuf/protobuf_test-unittest_lazy_dependencies_custom_option.pb.o
  CXX      google/protobuf/protobuf_test-unittest_lazy_dependencies_enum.pb.o
  CXX      google/protobuf/protobuf_test-unittest_lite_imports_nonlite.pb.o
  CXX      google/protobuf/protobuf_test-unittest_mset.pb.o
  CXX      google/protobuf/protobuf_test-unittest_mset_wire_format.pb.o
  CXX      google/protobuf/protobuf_test-unittest_no_arena_import.pb.o
  CXX      google/protobuf/protobuf_test-unittest_no_arena.pb.o
  CXX      google/protobuf/protobuf_test-unittest_no_field_presence.pb.o
  CXX      google/protobuf/protobuf_test-unittest_no_generic_services.pb.o
  CXX      google/protobuf/protobuf_test-unittest_optimize_for.pb.o
  CXX      google/protobuf/protobuf_test-unittest.pb.o
  CXX      google/protobuf/protobuf_test-unittest_preserve_unknown_enum2.pb.o
  CXX      google/protobuf/protobuf_test-unittest_preserve_unknown_enum.pb.o
  CXX      google/protobuf/protobuf_test-unittest_proto3.pb.o
  CXX      google/protobuf/protobuf_test-unittest_proto3_arena.pb.o
  CXX      google/protobuf/protobuf_test-unittest_proto3_arena_lite.pb.o
  CXX      google/protobuf/protobuf_test-unittest_proto3_lite.pb.o
  CXX      google/protobuf/protobuf_test-unittest_proto3_optional.pb.o
  CXX      google/protobuf/protobuf_test-unittest_well_known_types.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-anys.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-books.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-default_value.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-default_value_test.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-field_mask.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-maps.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-oneofs.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-proto3.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-struct.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-timestamp_duration.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_test-wrappers.pb.o
  CXX      google/protobuf/util/protobuf_test-json_format.pb.o
  CXX      google/protobuf/util/protobuf_test-json_format_proto3.pb.o
  CXX      google/protobuf/util/protobuf_test-message_differencer_unittest.pb.o
  CXXLD    protobuf-test
google/protobuf/testing/protobuf_test-googletest.o: In function `google::protobuf::(anonymous namespace)::GetTemporaryDirectoryName()':
/home/hezhiwen/work/protobuf/src/google/protobuf/testing/googletest.cc:124: warning: the use of `tmpnam' is dangerous, better use `mkstemp'
  CXX      google/protobuf/compiler/cpp/protobuf_lazy_descriptor_test-cpp_unittest.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-arena_test_util.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-test_util.o
  CXX      google/protobuf/testing/protobuf_lazy_descriptor_test-googletest.o
  CXX      google/protobuf/testing/protobuf_lazy_descriptor_test-file.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-map_lite_unittest.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_lite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_no_arena_lite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_import_lite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_import_public_lite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-any_test.pb.o
  CXX      google/protobuf/compiler/cpp/protobuf_lazy_descriptor_test-cpp_test_bad_identifiers.pb.o
  CXX      google/protobuf/compiler/cpp/protobuf_lazy_descriptor_test-cpp_test_large_enum_value.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-map_proto2_unittest.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-map_unittest.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_arena.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_custom_options.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_drop_unknown_fields.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_embed_optimize_for.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_empty.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_enormous_descriptor.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_import.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_import_public.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_lazy_dependencies.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_lazy_dependencies_custom_option.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_lazy_dependencies_enum.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_lite_imports_nonlite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_mset.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_mset_wire_format.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_no_arena_import.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_no_arena.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_no_field_presence.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_no_generic_services.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_optimize_for.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_preserve_unknown_enum2.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_preserve_unknown_enum.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_proto3.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_proto3_arena.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_proto3_arena_lite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_proto3_lite.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_proto3_optional.pb.o
  CXX      google/protobuf/protobuf_lazy_descriptor_test-unittest_well_known_types.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-anys.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-books.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-default_value.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-default_value_test.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-field_mask.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-maps.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-oneofs.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-proto3.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-struct.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-timestamp_duration.pb.o
  CXX      google/protobuf/util/internal/testdata/protobuf_lazy_descriptor_test-wrappers.pb.o
  CXX      google/protobuf/util/protobuf_lazy_descriptor_test-json_format.pb.o
  CXX      google/protobuf/util/protobuf_lazy_descriptor_test-json_format_proto3.pb.o
  CXX      google/protobuf/util/protobuf_lazy_descriptor_test-message_differencer_unittest.pb.o
  CXXLD    protobuf-lazy-descriptor-test
google/protobuf/testing/protobuf_lazy_descriptor_test-googletest.o: In function `google::protobuf::(anonymous namespace)::GetTemporaryDirectoryName()':
/home/hezhiwen/work/protobuf/src/google/protobuf/testing/googletest.cc:124: warning: the use of `tmpnam' is dangerous, better use `mkstemp'
  CXX      google/protobuf/protobuf_lite_test-lite_unittest.o
google/protobuf/lite_unittest.cc: In member function ‘virtual void google::protobuf::Lite_AllLite28_Test::TestBody()’:
google/protobuf/lite_unittest.cc:616:34: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
     int size = message1.ByteSize();
                                  ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_lite_unittest.pb.h:26,
                 from ./google/protobuf/map_lite_test_util.h:34,
                 from google/protobuf/lite_unittest.cc:39:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
google/protobuf/lite_unittest.cc: In member function ‘virtual void google::protobuf::Lite_AllLite29_Test::TestBody()’:
google/protobuf/lite_unittest.cc:633:34: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
     int size = message1.ByteSize();
                                  ^
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_lite_unittest.pb.h:26,
                 from ./google/protobuf/map_lite_test_util.h:34,
                 from google/protobuf/lite_unittest.cc:39:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_lite_test-arena_test_util.o
  CXX      google/protobuf/protobuf_lite_test-map_lite_test_util.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from google/protobuf/map_lite_test_util.cc:33:
./google/protobuf/map_test_util_impl.h: In instantiation of ‘static void google::protobuf::MapTestUtilImpl::ExpectMapFieldsSetInitialized(const MapMessage&) [with EnumType = protobuf_unittest::MapEnumLite; EnumType enum_value = (protobuf_unittest::MapEnumLite)0; MapMessage = protobuf_unittest::TestMapLite]’:
google/protobuf/map_lite_test_util.cc:81:14:   required from here
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:149:28: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   GTEST_ASSERT_(pred_format(#v1, #v2, v1, v2), \
                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/map_test_util_impl.h:413:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.map_int32_foreign_message().at(0).ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_lite_unittest.pb.h:26,
                 from ./google/protobuf/map_lite_test_util.h:34,
                 from google/protobuf/map_lite_test_util.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_lite_test-test_util_lite.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from google/protobuf/test_util_lite.cc:39:
google/protobuf/test_util_lite.cc: In static member function ‘static void google::protobuf::TestUtilLite::ExpectExtensionsClear(const protobuf_unittest::TestAllExtensionsLite&)’:
google/protobuf/test_util_lite.cc:1354:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/test_util_lite.cc:1354:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_lite.pb.h:26,
                 from ./google/protobuf/test_util_lite.h:38,
                 from google/protobuf/test_util_lite.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_lite_test-map_lite_unittest.pb.o
  CXX      google/protobuf/protobuf_lite_test-unittest_lite.pb.o
  CXX      google/protobuf/protobuf_lite_test-unittest_no_arena_lite.pb.o
  CXX      google/protobuf/protobuf_lite_test-unittest_import_lite.pb.o
  CXX      google/protobuf/protobuf_lite_test-unittest_import_public_lite.pb.o
  CXXLD    protobuf-lite-test
  CXX      google/protobuf/compiler/test_plugin-mock_code_generator.o
  CXX      google/protobuf/testing/test_plugin-file.o
  CXX      google/protobuf/compiler/test_plugin-test_plugin.o
  CXXLD    test_plugin
  CXX      google/protobuf/protobuf_lite_arena_test-lite_arena_unittest.o
  CXX      google/protobuf/protobuf_lite_arena_test-arena_test_util.o
  CXX      google/protobuf/protobuf_lite_arena_test-map_lite_test_util.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from ./google/protobuf/map_test_util_impl.h:36,
                 from google/protobuf/map_lite_test_util.cc:33:
./google/protobuf/map_test_util_impl.h: In instantiation of ‘static void google::protobuf::MapTestUtilImpl::ExpectMapFieldsSetInitialized(const MapMessage&) [with EnumType = protobuf_unittest::MapEnumLite; EnumType enum_value = (protobuf_unittest::MapEnumLite)0; MapMessage = protobuf_unittest::TestMapLite]’:
google/protobuf/map_lite_test_util.cc:81:14:   required from here
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:149:28: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   GTEST_ASSERT_(pred_format(#v1, #v2, v1, v2), \
                            ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
./google/protobuf/map_test_util_impl.h:413:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.map_int32_foreign_message().at(0).ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/map_lite_unittest.pb.h:26,
                 from ./google/protobuf/map_lite_test_util.h:34,
                 from google/protobuf/map_lite_test_util.cc:31:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_lite_arena_test-test_util_lite.o
In file included from ./../third_party/googletest/googletest/include/gtest/gtest.h:388:0,
                 from google/protobuf/test_util_lite.cc:39:
google/protobuf/test_util_lite.cc: In static member function ‘static void google::protobuf::TestUtilLite::ExpectExtensionsClear(const protobuf_unittest::TestAllExtensionsLite&)’:
google/protobuf/test_util_lite.cc:1354:33: warning: ‘int google::protobuf::MessageLite::ByteSize() const’ is deprecated: Please use ByteSizeLong() instead [-Wdeprecated-declarations]
   EXPECT_EQ(0, message.ByteSize());
                                 ^
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:77:52: note: in definition of macro ‘GTEST_ASSERT_’
   if (const ::testing::AssertionResult gtest_ar = (expression)) \
                                                    ^~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest_pred_impl.h:164:3: note: in expansion of macro ‘GTEST_PRED_FORMAT2_’
   GTEST_PRED_FORMAT2_(pred_format, v1, v2, GTEST_NONFATAL_FAILURE_)
   ^~~~~~~~~~~~~~~~~~~
./../third_party/googletest/googletest/include/gtest/gtest.h:2049:3: note: in expansion of macro ‘EXPECT_PRED_FORMAT2’
   EXPECT_PRED_FORMAT2(::testing::internal:: \
   ^~~~~~~~~~~~~~~~~~~
google/protobuf/test_util_lite.cc:1354:3: note: in expansion of macro ‘EXPECT_EQ’
   EXPECT_EQ(0, message.ByteSize());
   ^~~~~~~~~
In file included from ./google/protobuf/generated_enum_util.h:36:0,
                 from ./google/protobuf/map.h:49,
                 from ./google/protobuf/generated_message_table_driven.h:34,
                 from ./google/protobuf/unittest_lite.pb.h:26,
                 from ./google/protobuf/test_util_lite.h:38,
                 from google/protobuf/test_util_lite.cc:35:
./google/protobuf/message_lite.h:405:7: note: declared here
   int ByteSize() const { return internal::ToIntSize(ByteSizeLong()); }
       ^~~~~~~~
  CXX      google/protobuf/protobuf_lite_arena_test-map_lite_unittest.pb.o
  CXX      google/protobuf/protobuf_lite_arena_test-unittest_lite.pb.o
  CXX      google/protobuf/protobuf_lite_arena_test-unittest_no_arena_lite.pb.o
  CXX      google/protobuf/protobuf_lite_arena_test-unittest_import_lite.pb.o
  CXX      google/protobuf/protobuf_lite_arena_test-unittest_import_public_lite.pb.o
  CXXLD    protobuf-lite-arena-test
echo "// Generated from Makefile.am" > no_warning_test.cc
for FILE in google/protobuf/stubs/callback.h google/protobuf/stubs/bytestream.h google/protobuf/stubs/casts.h google/protobuf/stubs/common.h google/protobuf/stubs/fastmem.h google/protobuf/stubs/hash.h google/protobuf/stubs/logging.h google/protobuf/stubs/macros.h google/protobuf/stubs/map_util.h google/protobuf/stubs/mutex.h google/protobuf/stubs/once.h google/protobuf/stubs/platform_macros.h google/protobuf/stubs/port.h google/protobuf/stubs/status.h google/protobuf/stubs/stl_util.h google/protobuf/stubs/stringpiece.h google/protobuf/stubs/strutil.h google/protobuf/stubs/template_util.h google/protobuf/any.pb.h google/protobuf/api.pb.h google/protobuf/any.h google/protobuf/arena.h google/protobuf/arena_impl.h google/protobuf/arenastring.h google/protobuf/descriptor_database.h google/protobuf/descriptor.h google/protobuf/descriptor.pb.h google/protobuf/duration.pb.h google/protobuf/dynamic_message.h google/protobuf/empty.pb.h google/protobuf/extension_set.h google/protobuf/extension_set_inl.h google/protobuf/field_mask.pb.h google/protobuf/generated_enum_reflection.h google/protobuf/generated_enum_util.h google/protobuf/generated_message_reflection.h google/protobuf/generated_message_table_driven.h google/protobuf/generated_message_util.h google/protobuf/has_bits.h google/protobuf/implicit_weak_message.h google/protobuf/inlined_string_field.h google/protobuf/io/io_win32.h google/protobuf/map_entry.h google/protobuf/map_entry_lite.h google/protobuf/map_field.h google/protobuf/map_field_inl.h google/protobuf/map_field_lite.h google/protobuf/map.h google/protobuf/map_type_handler.h google/protobuf/message.h google/protobuf/message_lite.h google/protobuf/metadata.h google/protobuf/metadata_lite.h google/protobuf/parse_context.h google/protobuf/port.h google/protobuf/port_def.inc google/protobuf/port_undef.inc google/protobuf/reflection.h google/protobuf/reflection_ops.h google/protobuf/repeated_field.h google/protobuf/service.h google/protobuf/source_context.pb.h google/protobuf/struct.pb.h google/protobuf/text_format.h google/protobuf/timestamp.pb.h google/protobuf/type.pb.h google/protobuf/unknown_field_set.h google/protobuf/wire_format.h google/protobuf/wire_format_lite.h google/protobuf/wrappers.pb.h google/protobuf/io/coded_stream.h  google/protobuf/io/printer.h google/protobuf/io/strtod.h google/protobuf/io/tokenizer.h google/protobuf/io/zero_copy_stream.h google/protobuf/io/zero_copy_stream_impl.h google/protobuf/io/zero_copy_stream_impl_lite.h google/protobuf/compiler/code_generator.h google/protobuf/compiler/command_line_interface.h google/protobuf/compiler/importer.h google/protobuf/compiler/parser.h google/protobuf/compiler/plugin.h google/protobuf/compiler/plugin.pb.h google/protobuf/compiler/cpp/cpp_generator.h google/protobuf/compiler/csharp/csharp_generator.h google/protobuf/compiler/csharp/csharp_names.h google/protobuf/compiler/java/java_generator.h google/protobuf/compiler/java/java_names.h google/protobuf/compiler/js/js_generator.h google/protobuf/compiler/js/well_known_types_embed.h google/protobuf/compiler/objectivec/objectivec_generator.h google/protobuf/compiler/objectivec/objectivec_helpers.h google/protobuf/compiler/php/php_generator.h google/protobuf/compiler/python/python_generator.h google/protobuf/compiler/ruby/ruby_generator.h google/protobuf/util/type_resolver.h google/protobuf/util/delimited_message_util.h google/protobuf/util/field_comparator.h google/protobuf/util/field_mask_util.h google/protobuf/util/json_util.h google/protobuf/util/time_util.h google/protobuf/util/type_resolver_util.h google/protobuf/util/message_differencer.h; do \
    echo "#include <${FILE}>" >> no_warning_test.cc; \
done
echo "int main(int, char**) { return 0; }" >> no_warning_test.cc
  CXX      no_warning_test-no_warning_test.o
  CXX      google/protobuf/no_warning_test-map_lite_unittest.pb.o
  CXX      google/protobuf/no_warning_test-unittest_lite.pb.o
  CXX      google/protobuf/no_warning_test-unittest_no_arena_lite.pb.o
  CXX      google/protobuf/no_warning_test-unittest_import_lite.pb.o
  CXX      google/protobuf/no_warning_test-unittest_import_public_lite.pb.o
  CXX      google/protobuf/no_warning_test-any_test.pb.o
  CXX      google/protobuf/compiler/cpp/no_warning_test-cpp_test_bad_identifiers.pb.o
  CXX      google/protobuf/compiler/cpp/no_warning_test-cpp_test_large_enum_value.pb.o
  CXX      google/protobuf/no_warning_test-map_proto2_unittest.pb.o
  CXX      google/protobuf/no_warning_test-map_unittest.pb.o
  CXX      google/protobuf/no_warning_test-unittest_arena.pb.o
  CXX      google/protobuf/no_warning_test-unittest_custom_options.pb.o
  CXX      google/protobuf/no_warning_test-unittest_drop_unknown_fields.pb.o
  CXX      google/protobuf/no_warning_test-unittest_embed_optimize_for.pb.o
  CXX      google/protobuf/no_warning_test-unittest_empty.pb.o
  CXX      google/protobuf/no_warning_test-unittest_enormous_descriptor.pb.o
  CXX      google/protobuf/no_warning_test-unittest_import.pb.o
  CXX      google/protobuf/no_warning_test-unittest_import_public.pb.o
  CXX      google/protobuf/no_warning_test-unittest_lazy_dependencies.pb.o
  CXX      google/protobuf/no_warning_test-unittest_lazy_dependencies_custom_option.pb.o
  CXX      google/protobuf/no_warning_test-unittest_lazy_dependencies_enum.pb.o
  CXX      google/protobuf/no_warning_test-unittest_lite_imports_nonlite.pb.o
  CXX      google/protobuf/no_warning_test-unittest_mset.pb.o
  CXX      google/protobuf/no_warning_test-unittest_mset_wire_format.pb.o
  CXX      google/protobuf/no_warning_test-unittest_no_arena_import.pb.o
  CXX      google/protobuf/no_warning_test-unittest_no_arena.pb.o
  CXX      google/protobuf/no_warning_test-unittest_no_field_presence.pb.o
  CXX      google/protobuf/no_warning_test-unittest_no_generic_services.pb.o
  CXX      google/protobuf/no_warning_test-unittest_optimize_for.pb.o
  CXX      google/protobuf/no_warning_test-unittest.pb.o
  CXX      google/protobuf/no_warning_test-unittest_preserve_unknown_enum2.pb.o
  CXX      google/protobuf/no_warning_test-unittest_preserve_unknown_enum.pb.o
  CXX      google/protobuf/no_warning_test-unittest_proto3.pb.o
  CXX      google/protobuf/no_warning_test-unittest_proto3_arena.pb.o
  CXX      google/protobuf/no_warning_test-unittest_proto3_arena_lite.pb.o
  CXX      google/protobuf/no_warning_test-unittest_proto3_lite.pb.o
  CXX      google/protobuf/no_warning_test-unittest_proto3_optional.pb.o
  CXX      google/protobuf/no_warning_test-unittest_well_known_types.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-anys.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-books.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-default_value.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-default_value_test.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-field_mask.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-maps.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-oneofs.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-proto3.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-struct.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-timestamp_duration.pb.o
  CXX      google/protobuf/util/internal/testdata/no_warning_test-wrappers.pb.o
  CXX      google/protobuf/util/no_warning_test-json_format.pb.o
  CXX      google/protobuf/util/no_warning_test-json_format_proto3.pb.o
  CXX      google/protobuf/util/no_warning_test-message_differencer_unittest.pb.o
  CXXLD    no-warning-test
make[2]: 离开目录“/home/hezhiwen/work/protobuf/src”
make  check-TESTS
make[2]: 进入目录“/home/hezhiwen/work/protobuf/src”
make[3]: 进入目录“/home/hezhiwen/work/protobuf/src”
PASS: protobuf-test
PASS: protobuf-lazy-descriptor-test
PASS: protobuf-lite-test
PASS: google/protobuf/compiler/zip_output_unittest.sh
PASS: protobuf-lite-arena-test
PASS: no-warning-test
============================================================================
Testsuite summary for Protocol Buffers 3.11.4
============================================================================
# TOTAL: 6
# PASS:  6
# SKIP:  0
# XFAIL: 0
# FAIL:  0
# XPASS: 0
# ERROR: 0
============================================================================
make[3]: 离开目录“/home/hezhiwen/work/protobuf/src”
make[2]: 离开目录“/home/hezhiwen/work/protobuf/src”
make[1]: 离开目录“/home/hezhiwen/work/protobuf/src”

# output of make install

Making install in .
make[1]: 进入目录“/home/hezhiwen/work/protobuf”
make[2]: 进入目录“/home/hezhiwen/work/protobuf”
make[2]: 对“install-exec-am”无需做任何事。
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/lib/pkgconfig'
 /usr/bin/install -c -m 644 protobuf.pc protobuf-lite.pc '/home/hezhiwen/bin/protobuf/lib/pkgconfig'
make[2]: 离开目录“/home/hezhiwen/work/protobuf”
make[1]: 离开目录“/home/hezhiwen/work/protobuf”
Making install in src
make[1]: 进入目录“/home/hezhiwen/work/protobuf/src”
make[2]: 进入目录“/home/hezhiwen/work/protobuf/src”
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/lib'
 /bin/bash ../libtool   --mode=install /usr/bin/install -c   libprotobuf-lite.la libprotobuf.la libprotoc.la '/home/hezhiwen/bin/protobuf/lib'
libtool: install: /usr/bin/install -c .libs/libprotobuf-lite.so.22.0.4 /home/hezhiwen/bin/protobuf/lib/libprotobuf-lite.so.22.0.4
libtool: install: (cd /home/hezhiwen/bin/protobuf/lib && { ln -s -f libprotobuf-lite.so.22.0.4 libprotobuf-lite.so.22 || { rm -f libprotobuf-lite.so.22 && ln -s libprotobuf-lite.so.22.0.4 libprotobuf-lite.so.22; }; })
libtool: install: (cd /home/hezhiwen/bin/protobuf/lib && { ln -s -f libprotobuf-lite.so.22.0.4 libprotobuf-lite.so || { rm -f libprotobuf-lite.so && ln -s libprotobuf-lite.so.22.0.4 libprotobuf-lite.so; }; })
libtool: install: /usr/bin/install -c .libs/libprotobuf-lite.lai /home/hezhiwen/bin/protobuf/lib/libprotobuf-lite.la
libtool: install: /usr/bin/install -c .libs/libprotobuf.so.22.0.4 /home/hezhiwen/bin/protobuf/lib/libprotobuf.so.22.0.4
libtool: install: (cd /home/hezhiwen/bin/protobuf/lib && { ln -s -f libprotobuf.so.22.0.4 libprotobuf.so.22 || { rm -f libprotobuf.so.22 && ln -s libprotobuf.so.22.0.4 libprotobuf.so.22; }; })
libtool: install: (cd /home/hezhiwen/bin/protobuf/lib && { ln -s -f libprotobuf.so.22.0.4 libprotobuf.so || { rm -f libprotobuf.so && ln -s libprotobuf.so.22.0.4 libprotobuf.so; }; })
libtool: install: /usr/bin/install -c .libs/libprotobuf.lai /home/hezhiwen/bin/protobuf/lib/libprotobuf.la
libtool: warning: relinking 'libprotoc.la'
libtool: install: (cd /home/hezhiwen/work/protobuf/src; /bin/bash "/home/hezhiwen/work/protobuf/libtool"  --silent --tag CXX --mode=relink g++ -pthread -DHAVE_PTHREAD=1 -Wall -Wno-sign-compare -O2 -g -std=c++11 -DNDEBUG -version-info 22:4:0 -export-dynamic -no-undefined -Wl,--version-script=./libprotoc.map -o libprotoc.la -rpath /home/hezhiwen/bin/protobuf/lib google/protobuf/compiler/code_generator.lo google/protobuf/compiler/command_line_interface.lo google/protobuf/compiler/plugin.lo google/protobuf/compiler/plugin.pb.lo google/protobuf/compiler/subprocess.lo google/protobuf/compiler/zip_writer.lo google/protobuf/compiler/cpp/cpp_enum.lo google/protobuf/compiler/cpp/cpp_enum_field.lo google/protobuf/compiler/cpp/cpp_extension.lo google/protobuf/compiler/cpp/cpp_field.lo google/protobuf/compiler/cpp/cpp_file.lo google/protobuf/compiler/cpp/cpp_generator.lo google/protobuf/compiler/cpp/cpp_helpers.lo google/protobuf/compiler/cpp/cpp_map_field.lo google/protobuf/compiler/cpp/cpp_message.lo google/protobuf/compiler/cpp/cpp_message_field.lo google/protobuf/compiler/cpp/cpp_padding_optimizer.lo google/protobuf/compiler/cpp/cpp_primitive_field.lo google/protobuf/compiler/cpp/cpp_service.lo google/protobuf/compiler/cpp/cpp_string_field.lo google/protobuf/compiler/java/java_context.lo google/protobuf/compiler/java/java_enum.lo google/protobuf/compiler/java/java_enum_lite.lo google/protobuf/compiler/java/java_enum_field.lo google/protobuf/compiler/java/java_enum_field_lite.lo google/protobuf/compiler/java/java_extension.lo google/protobuf/compiler/java/java_extension_lite.lo google/protobuf/compiler/java/java_field.lo google/protobuf/compiler/java/java_file.lo google/protobuf/compiler/java/java_generator.lo google/protobuf/compiler/java/java_generator_factory.lo google/protobuf/compiler/java/java_helpers.lo google/protobuf/compiler/java/java_map_field.lo google/protobuf/compiler/java/java_map_field_lite.lo google/protobuf/compiler/java/java_message.lo google/protobuf/compiler/java/java_message_lite.lo google/protobuf/compiler/java/java_message_builder.lo google/protobuf/compiler/java/java_message_builder_lite.lo google/protobuf/compiler/java/java_message_field.lo google/protobuf/compiler/java/java_message_field_lite.lo google/protobuf/compiler/java/java_name_resolver.lo google/protobuf/compiler/java/java_primitive_field.lo google/protobuf/compiler/java/java_primitive_field_lite.lo google/protobuf/compiler/java/java_shared_code_generator.lo google/protobuf/compiler/java/java_service.lo google/protobuf/compiler/java/java_string_field.lo google/protobuf/compiler/java/java_string_field_lite.lo google/protobuf/compiler/java/java_doc_comment.lo google/protobuf/compiler/js/js_generator.lo google/protobuf/compiler/js/well_known_types_embed.lo google/protobuf/compiler/objectivec/objectivec_enum.lo google/protobuf/compiler/objectivec/objectivec_enum_field.lo google/protobuf/compiler/objectivec/objectivec_extension.lo google/protobuf/compiler/objectivec/objectivec_field.lo google/protobuf/compiler/objectivec/objectivec_file.lo google/protobuf/compiler/objectivec/objectivec_generator.lo google/protobuf/compiler/objectivec/objectivec_helpers.lo google/protobuf/compiler/objectivec/objectivec_map_field.lo google/protobuf/compiler/objectivec/objectivec_message.lo google/protobuf/compiler/objectivec/objectivec_message_field.lo google/protobuf/compiler/objectivec/objectivec_oneof.lo google/protobuf/compiler/objectivec/objectivec_primitive_field.lo google/protobuf/compiler/php/php_generator.lo google/protobuf/compiler/python/python_generator.lo google/protobuf/compiler/ruby/ruby_generator.lo google/protobuf/compiler/csharp/csharp_doc_comment.lo google/protobuf/compiler/csharp/csharp_enum.lo google/protobuf/compiler/csharp/csharp_enum_field.lo google/protobuf/compiler/csharp/csharp_field_base.lo google/protobuf/compiler/csharp/csharp_generator.lo google/protobuf/compiler/csharp/csharp_helpers.lo google/protobuf/compiler/csharp/csharp_map_field.lo google/protobuf/compiler/csharp/csharp_message.lo google/protobuf/compiler/csharp/csharp_message_field.lo google/protobuf/compiler/csharp/csharp_primitive_field.lo google/protobuf/compiler/csharp/csharp_reflection_class.lo google/protobuf/compiler/csharp/csharp_repeated_enum_field.lo google/protobuf/compiler/csharp/csharp_repeated_message_field.lo google/protobuf/compiler/csharp/csharp_repeated_primitive_field.lo google/protobuf/compiler/csharp/csharp_source_generator_base.lo google/protobuf/compiler/csharp/csharp_wrapper_field.lo libprotobuf.la )
libtool: install: /usr/bin/install -c .libs/libprotoc.so.22.0.4T /home/hezhiwen/bin/protobuf/lib/libprotoc.so.22.0.4
libtool: install: (cd /home/hezhiwen/bin/protobuf/lib && { ln -s -f libprotoc.so.22.0.4 libprotoc.so.22 || { rm -f libprotoc.so.22 && ln -s libprotoc.so.22.0.4 libprotoc.so.22; }; })
libtool: install: (cd /home/hezhiwen/bin/protobuf/lib && { ln -s -f libprotoc.so.22.0.4 libprotoc.so || { rm -f libprotoc.so && ln -s libprotoc.so.22.0.4 libprotoc.so; }; })
libtool: install: /usr/bin/install -c .libs/libprotoc.lai /home/hezhiwen/bin/protobuf/lib/libprotoc.la
libtool: install: /usr/bin/install -c .libs/libprotobuf-lite.a /home/hezhiwen/bin/protobuf/lib/libprotobuf-lite.a
libtool: install: chmod 644 /home/hezhiwen/bin/protobuf/lib/libprotobuf-lite.a
libtool: install: ranlib /home/hezhiwen/bin/protobuf/lib/libprotobuf-lite.a
libtool: install: /usr/bin/install -c .libs/libprotobuf.a /home/hezhiwen/bin/protobuf/lib/libprotobuf.a
libtool: install: chmod 644 /home/hezhiwen/bin/protobuf/lib/libprotobuf.a
libtool: install: ranlib /home/hezhiwen/bin/protobuf/lib/libprotobuf.a
libtool: install: /usr/bin/install -c .libs/libprotoc.a /home/hezhiwen/bin/protobuf/lib/libprotoc.a
libtool: install: chmod 644 /home/hezhiwen/bin/protobuf/lib/libprotoc.a
libtool: install: ranlib /home/hezhiwen/bin/protobuf/lib/libprotoc.a
libtool: finish: PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/sbin" ldconfig -n /home/hezhiwen/bin/protobuf/lib
----------------------------------------------------------------------
Libraries have been installed in:
   /home/hezhiwen/bin/protobuf/lib

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
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/bin'
  /bin/bash ../libtool   --mode=install /usr/bin/install -c protoc '/home/hezhiwen/bin/protobuf/bin'
libtool: install: /usr/bin/install -c .libs/protoc /home/hezhiwen/bin/protobuf/bin/protoc
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf'
 /usr/bin/install -c -m 644  google/protobuf/descriptor.proto google/protobuf/any.proto google/protobuf/api.proto google/protobuf/duration.proto google/protobuf/empty.proto google/protobuf/field_mask.proto google/protobuf/source_context.proto google/protobuf/struct.proto google/protobuf/timestamp.proto google/protobuf/type.proto google/protobuf/wrappers.proto '/home/hezhiwen/bin/protobuf/include/google/protobuf'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler'
 /usr/bin/install -c -m 644  google/protobuf/compiler/plugin.proto '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf'
 /usr/bin/install -c -m 644  google/protobuf/any.pb.h google/protobuf/api.pb.h google/protobuf/any.h google/protobuf/arena.h google/protobuf/arena_impl.h google/protobuf/arenastring.h google/protobuf/descriptor_database.h google/protobuf/descriptor.h google/protobuf/descriptor.pb.h google/protobuf/duration.pb.h google/protobuf/dynamic_message.h google/protobuf/empty.pb.h google/protobuf/extension_set.h google/protobuf/extension_set_inl.h google/protobuf/field_mask.pb.h google/protobuf/generated_enum_reflection.h google/protobuf/generated_enum_util.h google/protobuf/generated_message_reflection.h google/protobuf/generated_message_table_driven.h google/protobuf/generated_message_util.h google/protobuf/has_bits.h google/protobuf/implicit_weak_message.h google/protobuf/inlined_string_field.h google/protobuf/map_entry.h google/protobuf/map_entry_lite.h google/protobuf/map_field.h google/protobuf/map_field_inl.h google/protobuf/map_field_lite.h google/protobuf/map.h google/protobuf/map_type_handler.h google/protobuf/message.h google/protobuf/message_lite.h google/protobuf/metadata.h google/protobuf/metadata_lite.h google/protobuf/parse_context.h google/protobuf/port.h google/protobuf/port_def.inc google/protobuf/port_undef.inc google/protobuf/reflection.h google/protobuf/reflection_ops.h '/home/hezhiwen/bin/protobuf/include/google/protobuf'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/js'
 /usr/bin/install -c -m 644  google/protobuf/compiler/js/js_generator.h google/protobuf/compiler/js/well_known_types_embed.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/js'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/cpp'
 /usr/bin/install -c -m 644  google/protobuf/compiler/cpp/cpp_generator.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/cpp'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/ruby'
 /usr/bin/install -c -m 644  google/protobuf/compiler/ruby/ruby_generator.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/ruby'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/python'
 /usr/bin/install -c -m 644  google/protobuf/compiler/python/python_generator.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/python'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/util'
 /usr/bin/install -c -m 644  google/protobuf/util/type_resolver.h google/protobuf/util/delimited_message_util.h google/protobuf/util/field_comparator.h google/protobuf/util/field_mask_util.h google/protobuf/util/json_util.h google/protobuf/util/time_util.h google/protobuf/util/type_resolver_util.h google/protobuf/util/message_differencer.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/util'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/io'
 /usr/bin/install -c -m 644  google/protobuf/io/io_win32.h google/protobuf/io/coded_stream.h google/protobuf/io/printer.h google/protobuf/io/strtod.h google/protobuf/io/tokenizer.h google/protobuf/io/zero_copy_stream.h google/protobuf/io/zero_copy_stream_impl.h google/protobuf/io/zero_copy_stream_impl_lite.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/io'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/csharp'
 /usr/bin/install -c -m 644  google/protobuf/compiler/csharp/csharp_generator.h google/protobuf/compiler/csharp/csharp_names.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/csharp'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/php'
 /usr/bin/install -c -m 644  google/protobuf/compiler/php/php_generator.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/php'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/java'
 /usr/bin/install -c -m 644  google/protobuf/compiler/java/java_generator.h google/protobuf/compiler/java/java_names.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/java'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/stubs'
 /usr/bin/install -c -m 644  google/protobuf/stubs/callback.h google/protobuf/stubs/bytestream.h google/protobuf/stubs/casts.h google/protobuf/stubs/common.h google/protobuf/stubs/fastmem.h google/protobuf/stubs/hash.h google/protobuf/stubs/logging.h google/protobuf/stubs/macros.h google/protobuf/stubs/map_util.h google/protobuf/stubs/mutex.h google/protobuf/stubs/once.h google/protobuf/stubs/platform_macros.h google/protobuf/stubs/port.h google/protobuf/stubs/status.h google/protobuf/stubs/stl_util.h google/protobuf/stubs/stringpiece.h google/protobuf/stubs/strutil.h google/protobuf/stubs/template_util.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/stubs'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf'
 /usr/bin/install -c -m 644  google/protobuf/repeated_field.h google/protobuf/service.h google/protobuf/source_context.pb.h google/protobuf/struct.pb.h google/protobuf/text_format.h google/protobuf/timestamp.pb.h google/protobuf/type.pb.h google/protobuf/unknown_field_set.h google/protobuf/wire_format.h google/protobuf/wire_format_lite.h google/protobuf/wrappers.pb.h '/home/hezhiwen/bin/protobuf/include/google/protobuf'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/objectivec'
 /usr/bin/install -c -m 644  google/protobuf/compiler/objectivec/objectivec_generator.h google/protobuf/compiler/objectivec/objectivec_helpers.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler/objectivec'
 /bin/mkdir -p '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler'
 /usr/bin/install -c -m 644  google/protobuf/compiler/code_generator.h google/protobuf/compiler/command_line_interface.h google/protobuf/compiler/importer.h google/protobuf/compiler/parser.h google/protobuf/compiler/plugin.h google/protobuf/compiler/plugin.pb.h '/home/hezhiwen/bin/protobuf/include/google/protobuf/compiler'
make[2]: 离开目录“/home/hezhiwen/work/protobuf/src”
make[1]: 离开目录“/home/hezhiwen/work/protobuf/src”
