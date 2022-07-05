OBJDIR := objdir
OBJS := $(addprefix $(OBJDIR)/,main.o foo.o bar.o)

$(OBJDIR)/%.o : %.c
		cl.exe /o $@ $<

all: $(OBJS)

$(OBJS): | $(OBJDIR)

$(OBJDIR):
		mkdir $(OBJDIR)