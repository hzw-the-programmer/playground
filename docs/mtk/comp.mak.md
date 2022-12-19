make\Comp.mak
```
ifdef $($(COMPONENT))
  MODULE_MAKEFILE := make\$(strip $($(COMPONENT)))\$(strip $(COMPONENT))\$(strip $(COMPONENT)).mak
else
  MODULE_MAKEFILE := make\$(strip $(COMPONENT))\$(strip $(COMPONENT)).mak
  // make\app\app.mak
endif
include $(MODULE_MAKEFILE)
```

```
.SUFFIXES:
.SUFFIXES: .obj .c .s .cpp .arm .ltp .det
```

```
TARGLIB       =  $(subst \,/,$(OBJSDIR))/lib/$(strip $(COMPONENT)).lib
// build\{customer}\gprs\MT6260o\lib\app.lib
COMPOBJS_DIR  =  $(subst \,/,$(OBJSDIR))/$(strip $(COMPONENT))
// build\{customer}\gprs\MT6260o\app
OBJ_ARCHIVE   =  $(OBJSDIR)\$(COMPONENT)\$(strip $(COMPONENT)).via
// 
OBJ_ARCHIVE_SORT  =  $(OBJSDIR)\$(COMPONENT)\$(strip $(COMPONENT))_sort.via
// 
TARGDEP       =  $(subst \,/,$(RULESDIR))/$(strip $(COMPONENT)).dep
// build\{customer}\gprs\MT6260r\app.dep
COMPDETS_DIR  =  $(subst \,/,$(RULESDIR))/$(strip $(COMPONENT))_dep
// build\{customer}\gprs\MT6260r\codegen_dep

ifdef $($(COMPONENT))
  COMPBUILD_DIR =  $(FIXPATH)\make\$(strip $($(COMPONENT)))\$(strip $(COMPONENT))
  COMPPARENT_DIR = $(FIXPATH)\make\$(strip $($(COMPONENT)))
else
  COMPBUILD_DIR =  $(FIXPATH)\make\$(strip $(COMPONENT))
  // make\app
endif

MODULE_FOLDER = $(TARGDIR)\module
// build\{customer}\module
ifdef $($(COMPONENT))
COMPONENT_FOLDER = $(MODULE_FOLDER)\$(strip $($(COMPONENT)))\$(strip $(COMPONENT))
else
COMPONENT_FOLDER = $(MODULE_FOLDER)\$(strip $(COMPONENT))
// build\{customer}\module\app
endif
COMPONENT_LOG = $(COMPONENT_FOLDER)\$(strip $(COMPONENT))
```

```
SRC_LIST := $(sort $(SRC_LIST))

ifdef SRC_RULE_FLAG32
  # SRC_RULE_FLAG32 means the asm file should be compiled by armasm using -32 flag
  SRCLIST1 := $(foreach file,$(SRC_LIST),$(if $(filter $(notdir $(call Upper,$(file))) $(call Upper,$(file)),$(call Upper,$(SRC_RULE_FLAG32))),$(file),))
  SRCLIST0 := $(foreach file,$(SRC_LIST),$(if $(filter $(notdir $(call Upper,$(file))) $(call Upper,$(file)),$(call Upper,$(SRC_RULE_FLAG32))),,$(file)))
else
  SRCLIST0 := $(SRC_LIST)
endif
ifdef SRC_RULE_PREPROCESS
  # SRC_RULE_PREPROCESS means the asm file needs to be preprocessed by armcc -E and then armasm
  SRCLIST2 := $(foreach file,$(SRCLIST0),$(if $(filter $(notdir $(call Upper,$(file))) $(call Upper,$(file)),$(call Upper,$(SRC_RULE_PREPROCESS))),$(file),))
  SRCLIST0 := $(foreach file,$(SRCLIST0),$(if $(filter $(notdir $(call Upper,$(file))) $(call Upper,$(file)),$(call Upper,$(SRC_RULE_PREPROCESS))),,$(file)))
endif

SRCLIST := $(subst .S,.s,$(subst .C,.c,$(SRC_LIST)))
CSRCS   := $(filter %.c, $(subst .C,.c,$(SRCLIST0)))
CPPSRCS := $(filter %.cpp, $(SRCLIST0))
ASRCS   := $(filter %.s, $(subst .S,.s,$(SRCLIST0)))
ASRCS1  := $(filter %.s, $(subst .S,.s,$(SRCLIST1)))
ASRCS2  := $(filter %.s, $(subst .S,.s,$(SRCLIST2)))
ARMSRCS := $(filter %.arm, $(SRCLIST0))
```

```
INCDIRS += $(INC_DIR)
INCDIRS := $(call uniq,$(INCDIRS))
```

```
#COMP_DEFS is from $module.mak
DEFINES += $(COMP_DEFS)
```

```
CINCDIRS 	=  $(foreach inc, $(subst \,/,$(INCDIRS)),-I$(inc))
```

```
CDEFS 		=  $(foreach def, $(DEFINES),-D$(def))
```

```
$(foreach argu,$(CPPSRCS), \
  $(eval CPPOBJ := $(patsubst %.cpp,%.obj, $(notdir $(subst \,/,$(argu))))) \
  $(eval CPPDET := $(patsubst %.cpp,%.det, $(notdir $(subst \,/,$(argu))))) \
  $(eval $(call target_compile_cpp_obj,$(argu),$(CPPOBJ))) \
  $(eval $(call target_scan_cpp_det,$(argu),$(CPPDET))) \
  $(eval $(TARGLIB): $(CPPOBJ)) \
  $(eval $(TARGDEP): $(CPPDET)) \
)

define target_compile_cpp_obj
$(2): $(1) $$(NEED_CHECK_DEPEND_LIST)
   # mbis time probe
	@if /I "$$(strip $$(MBIS_EN_OBJ_LOG))" EQU "TRUE" (@perl -e "print 'T_S,$$(@F),O,'. time . \"\n\"";>>$$(strip $$(TARGDIR))\log\mbis\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@))".mbis")
	@echo Compiling $$< ... >$$(strip $$(TARGDIR))\log\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).log
	@if $$(ACTION)== remake $$(CPP_CMPLR) $$(VIA) $$(strip $$(TARGDIR))\via\$$(strip $$(COMPONENT)).via $$(VIA) $$(strip $$(TARGDIR))\via\$$(strip $$(COMPONENT))_inc.via -c $$(CPLUSFLAGS) -o $$(COMPOBJS_DIR)/$$(notdir $$@) $$< 2>>$$(strip $$(TARGDIR))\log\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).log
	@if not $$(ACTION)==remake $$(CPP_CMPLR) $$(VIA) $$(strip $$(TARGDIR))\via\$$(strip $$(COMPONENT)).via $$(VIA) $$(strip $$(TARGDIR))\via\$$(strip $$(COMPONENT))_inc.via -c $$(CPLUSFLAGS) $$(depend) $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d -o $$(COMPOBJS_DIR)/$$(notdir $$@) $$< 2>>$$(strip $$(TARGDIR))\log\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).log
	@if not $$(ACTION)==remake if exist $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d perl .\tools\pack_dep.pl $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d tools\copy_mmi_include_h.bat> $$(RULESDIR)\$$(strip $$(COMPONENT))_dep\$$(basename $$(notdir $$@)).det
	@if not $$(ACTION)==remake if exist $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d del /f /q $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d > nul
   # mbis time probe
	@if /I "$$(strip $$(MBIS_EN_OBJ_LOG))" EQU "TRUE" (@perl -e "print 'T_E,$$(@F),O,'. time . \"\n\"";>>$$(strip $$(TARGDIR))\log\mbis\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@))".mbis")
endef

define target_scan_cpp_det
$(2): $(1)
   # mbis time probe
	@if /I "$$(strip $$(MBIS_EN_OBJ_LOG))" EQU "TRUE" (@perl -e "print 'T_S,$$(@F),O,'. time . \"\n\"";>>$$(strip $$(TARGDIR))\log\mbis\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@))".mbis")
	@echo Pre-compiling $$< ... >$$(strip $$(TARGDIR))\log\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).log
	@$$(CPP_CMPLR) $$(VIA) $$(strip $$(TARGDIR))\via\$$(strip $$(COMPONENT)).via $$(VIA) $$(strip $$(TARGDIR))\via\$$(strip $$(COMPONENT))_inc.via -M $$< >$$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d 2>>$$(strip $$(TARGDIR))\log\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).log
	@if exist $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d perl .\tools\pack_dep.pl $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d tools\copy_mmi_include_h.bat> $$(RULESDIR)\$$(strip $$(COMPONENT))_dep\$$(basename $$(notdir $$@)).det
	@if exist $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d del /f /q $$(strip $$(TARGDIR))\dep\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@)).d > nul
   # mbis time probe
	@if /I "$$(strip $$(MBIS_EN_OBJ_LOG))" EQU "TRUE" (@perl -e "print 'T_E,$$(@F),O,'. time . \"\n\"";>>$$(strip $$(TARGDIR))\log\mbis\$$(strip $$(COMPONENT))\$$(basename $$(notdir $$@))".mbis")
endef
```