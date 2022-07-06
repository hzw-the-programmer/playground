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

objects := $(patsubst %.c,%.obj,$(wildcard *.c))

main: $(objects)
	cl.exe /nologo $(objects)

%.obj: %.c
	cl.exe /nologo /c $<

clean:
	rm *.obj *.exe

# Setting environment for using Microsoft Visual Studio 2008 x86 tools.
# echo %lib%
# export lib=/C/'Program Files (x86)'/'Microsoft Visual Studio 9.0'/VC/ATLMFC/LIB:/C/'Program Files (x86)'/'Microsoft Visual Studio 9.0'/VC/LIB:/c/'Program Files'/'Microsoft SDKs'/Windows/v6.0A/lib
