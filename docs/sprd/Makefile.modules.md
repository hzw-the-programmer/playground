#
# Pre-defined Make Rules and Modules Specific Compiler Options
#
include Makefile.rules
include $(call GETMAKEPATH,$(COMPONENT))

#
# Vars For Building Target
#
TARGET				= $(COMPONENT).a
INCPATH 			= $(addprefix -I, $(COMMON_INCPATH))
INCPATH				+=$(addprefix -I, $(MINCPATH))

SRCPATH 			= $(MSRCPATH)
MOBJPATH			= $(BUILD_DIR)/obj/$(COMPONENT)
DEPPATH				= $(BUILD_DIR)/dep/$(COMPONENT)

OBJECTS1 			= $(patsubst %.c, %.o, $(SOURCES))
OBJECTS2 			+= $(patsubst %.cpp, %.o, $(OBJECTS1))
OBJECTS3 			+= $(patsubst %.cxx, %.o, $(OBJECTS2))
OBJECTS 			+= $(patsubst %.s, %.o, $(OBJECTS3))

#
# VPATH Settings For The Specific Modules
#
vpath
vpath %.h        $(subst \,/,$(INCPATH))
vpath %.cpp      $(subst \,/,$(SRCPATH))
vpath %.cxx      $(subst \,/,$(SRCPATH))
vpath %.c        $(subst \,/,$(SRCPATH))
vpath %.dat      $(subst \,/,$(SRCPATH))
vpath %.s        $(subst \,/,$(SRCPATH))
vpath %.o        $(BUILD_DIR)/obj/$(COMPONENT)
vpath %.d        $(BUILD_DIR)/dep/$(COMPONENT)
vpath %.dep      $(BUILD_DIR)/dep

vpath %.a        $(TARGET_PATH)


# Common Commands for Building Libs, C and Assemly Language


$(TARGET) : $(OBJECTS)
	@$(PERL) make/perl_script/rm_file.pl $(TARGET) $(TARGET_PATH)/
	cd $(MOBJPATH) && $(AR) $(ARFLAGS) ../../../../$(TARGET_PATH)/$(TARGET)  $(VIA) ./objectlist.txt
	@$(PERL) make/perl_script/rm_file.pl *.d.tmp ./$(DEPPATH)/

%.o %.d :%.c $(COMPONENT).dep
	@$(ECHO)
	@$(ECHO) Compiling Source File $<...
ifeq ($(strip $(OPEN_MODULE_SUPPORT)),TRUE)
	@-$(PERL) make/perl_script/rm_file.pl $(TARGET) $(BUILD_DIR)/lib/
else
	@-$(PERL) make/perl_script/rm_file.pl $(TARGET) lib/$(PROJECT)/
endif
	@-$(PERL) make/perl_script/rm_file.pl $*.o $(MOBJPATH)/
#	$(ECHO) $(CCOMPLAN) $(CFLAGS)  $(VIA) $(BUILD_DIR)/dep/$(COMPONENT)_C_MACRO_INC.txt -c   -I$(ARMINC)  $<  $(MDFLAGS) $(DEPPATH)/$*.d.tmp -o $(MOBJPATH)/$*.o
	$(CCOMPLAN) $(CFLAGS)  $(VIA) $(BUILD_DIR)/dep/$(COMPONENT)_C_MACRO_INC.txt -c   -I$(ARMINC)  $<  $(MDFLAGS) $(DEPPATH)/$*.d.tmp -o $(MOBJPATH)/$*.o  \
             $(if $(findstring 0,$(STOP)),, || $(ECHO) && $(ECHO) !!! Error !!! You can terminate batch job by Control+C !  && $(ECHO) && pause)
	@$(PERL) ./make/perl_script/path_process.pl ./$(DEPPATH)/$*.d.tmp  $(BUILD_DIR)/dep/$(COMPONENT).dep "$(MAKESHELL)" > ./$(DEPPATH)/$*.d