mk1 := $(wildcard *.mk)
$(info $(mk1))
mk2 := $(shell echo *.mk)
$(info $(mk2))

c1 := $(wildcard *.c)
$(info $(c1))
c2 := $(shell find . -name "*.c")
$(info $(c2))

wf := $(shell which find)
$(info $(wf))

# /usr/bin
# pwd -W
# C:/Program Files/Git/usr/bin

contents := $(shell cat nested.mk)
$(info $(contents))
