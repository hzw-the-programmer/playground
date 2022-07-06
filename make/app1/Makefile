# ./make.exe -p | grep "CFLAGS"
# LINK.c = $(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)
# COMPILE.c = $(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c

# ./make.exe -p | grep "\-o"
# OUTPUT_OPTION = -o $@

# CC := cl.exe
# CFLAGS := /nologo
# OUTPUT_OPTION = /Fo $@

# objects := $(patsubst %.c,%.obj,$(wildcard *.c))
# $(info $(objects))

# main: $(objects)
# 	$(info $@)
# 	$(info $<)
# 	cl.exe main $(objects)

# objects := $(patsubst %.c,%.obj,$(wildcard *.c))
objects := $(shell find . -name "*.c")
objects := $(foreach o,$(objects),$(notdir $(o)))
objects := $(patsubst %.c,%.obj,$(objects))

VPATH = src
# Setting environment for using Microsoft Visual Studio 2008 x86 tools.
# echo %lib%
# export lib=/C/'Program Files (x86)'/'Microsoft Visual Studio 9.0'/VC/ATLMFC/LIB:/C/'Program Files (x86)'/'Microsoft Visual Studio 9.0'/VC/LIB:/c/'Program Files'/'Microsoft SDKs'/Windows/v6.0A/lib

main: $(objects)
	cl.exe /nologo /Femain $(objects)

%.obj: %.c
	cl.exe /nologo /c $<

clean:
	-@rm *.obj *.exe 2>/dev/null || true

# rm *.obj *.exe
# $ ../make.exe clean
# rm *.obj *.exe
# rm: cannot remove '*.obj': No such file or directory
# rm: cannot remove '*.exe': No such file or directory
# make: *** [clean] Error 1

# @rm *.obj *.exe
# $ ../make.exe clean
# rm: cannot remove '*.obj': No such file or directory
# rm: cannot remove '*.exe': No such file or directory
# make: *** [clean] Error 1

# -@rm *.obj *.exe
# $ ../make.exe clean
# rm: cannot remove '*.obj': No such file or directory
# rm: cannot remove '*.exe': No such file or directory
# make: [clean] Error 1 (ignored)

# -@rm *.obj *.exe 2>/dev/null
# $ ../make.exe clean
# make: [clean] Error 1 (ignored)
