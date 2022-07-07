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

BUILD_DIR := build

# objects := $(patsubst %.c,%.obj,$(wildcard *.c))
objects := $(shell find . -name "*.c")
objects := $(foreach o,$(objects),$(notdir $(o)))
objects := $(patsubst %.c,%.obj,$(objects))
objects := $(addprefix $(BUILD_DIR)/,$(objects))
# $(error $(objects))

# https://www.gnu.org/software/make/manual/html_node/Recipes_002fSearch.html#Recipes_002fSearch
# https://www.gnu.org/software/make/manual/html_node/General-Search.html
# https://www.gnu.org/software/make/manual/html_node/Selective-Search.html
VPATH = src

# Setting environment for using Microsoft Visual Studio 2008 x86 tools.
# echo %lib%
# export lib=/C/'Program Files (x86)'/'Microsoft Visual Studio 9.0'/VC/ATLMFC/LIB:/C/'Program Files (x86)'/'Microsoft Visual Studio 9.0'/VC/LIB:/c/'Program Files'/'Microsoft SDKs'/Windows/v6.0A/lib

main: $(objects)
	cl.exe /nologo /Fe$(BUILD_DIR)/main $(objects)

$(BUILD_DIR)/%.obj: %.c | $(BUILD_DIR)
	cl.exe /nologo /c /Fo$@ $<

$(BUILD_DIR):
	mkdir $(BUILD_DIR)

clean:
	-@rm -fr $(BUILD_DIR) 2>/dev/null || true

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
