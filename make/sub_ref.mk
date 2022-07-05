foo := a.o b.o c.o o.o.o
bar := $(foo:.o=.c)
$(info $(bar))