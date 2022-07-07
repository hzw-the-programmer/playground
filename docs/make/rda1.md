```
############################ Implicit rules ########################################

# We generate one dependency file on the fly for each source file, 
# either when preprocessing for asm files, or when compiling for C files.
# Those depfiles are also board depend (to deal with conditional includes).
# Empty rules are generated for all header files, to avoid issues in case
# one header is deleted (-MP)

${OBJ_REL_PATH}/%.o: ${LOCAL_SRC_DIR}/%.S
	@${ECHO} "CPP               $*.S"
	$(CPP) $(CPPFLAGS) $(ASCPPFLAGS)  -MT ${OBJ_REL_PATH}/$*.o -MD -MP -MF ${DEPS_REL_PATH}/$*.d -o ${OBJ_REL_PATH}/$*.asm $(REALPATH)
	@${ECHO} "AS                $*.asm"
	$(AS) $(ASFLAGS) -o ${OBJ_REL_PATH}/$*.o ${OBJ_REL_PATH}/$*.asm

${OBJ_REL_PATH}/%.o: ${LOCAL_SRC_DIR}/%.c
	@${ECHO} "CC                $*.c"
	$(CC) -MT ${OBJ_REL_PATH}/$*.o -MD -MP -MF ${DEPS_REL_PATH}/$*.d $(C_SPECIFIC_CFLAGS) $(CFLAGS) $(CT_MIPS16_CFLAGS) $(MYCFLAGS) $(CPPFLAGS)  -o ${OBJ_REL_PATH}/$*.o $(REALPATH)

${OBJ_REL_PATH}/%.o: ${LOCAL_SRC_DIR}/%.cpp
	@${ECHO} "C++               $*.cpp"
	$(C++) -MT ${OBJ_REL_PATH}/$*.o -MD -MP -MF ${DEPS_REL_PATH}/$*.d $(C++_SPECIFIC_CFLAGS) $(CFLAGS) $(CT_MIPS16_CFLAGS) $(MYCFLAGS) $(CPPFLAGS)  -o ${OBJ_REL_PATH}/$*.o $(REALPATH) $(EXTERN_CPPFLAGS)
```

```
########################################################################
# MAKELEVEL=0 Things to do only once
# Variables that are defined only once should be in this section and 
# exported to sub-make
########################################################################
ifeq ($(MAKELEVEL),0)
...
##############################################################################
# Generic environment stuff
###############################################################################
## Generic directory names
...
export SRC_DIR := src
...
########################################################################
# End of MAKELEVEL=0. Things to do only once.
########################################################################
endif
```

```
#################################
## Include path generation ######
#################################
# LOCAL_API_DEPENDS
# list all the include from LOCAL_API_DEPENDS
DEPENDENCY_INCLUDE_PATH := ${foreach module, ${LOCAL_API_DEPENDS}, -I${SOFT_WORKDIR}/${module}/${INC_DIR} }

# ADD all the include from LOCAL_MODULE_DEPENDS 
# (if we depend on a module, we depend on its include also)
DEPENDENCY_INCLUDE_PATH += ${foreach module, ${LOCAL_MODULE_DEPENDS}, -I${SOFT_WORKDIR}/${module}/${INC_DIR} }

# List LOCAL_ADD_INCLUDE header retrieving path
MYINCLUDEDIR := ${foreach tmpDir, ${LOCAL_ADD_INCLUDE}, -I${SOFT_WORKDIR}/${tmpDir}}

# List LOCAL_ADD_ABS_INCLUDE (in absolute path) header retrieving path
MYINCLUDEDIR += ${foreach tmpDir, ${LOCAL_ADD_ABS_INCLUDE}, -I${tmpDir}}

# Root include directory
ROOT_INCLUDE	:= ${SOFT_WORKDIR}/${INC_DIR}

# Final include path
# ROOT_INCLUDE at the end because DEPENDENCY_INCLUDE_PATH must be allowed to supersede it (e.g. for sxs_type.h)
INCLUDE_PATH	:= ${MYINCLUDEDIR} -I${LOCAL_INC_DIR} -I${LOCAL_SRC_DIR} ${DEPENDENCY_INCLUDE_PATH}  -I${ROOT_INCLUDE}
```

```
grep "MMI_SUPPORT_NATIVE_JAVA" -r . --exclude-dir={application,env}
```

### add a new module
proj\target\target_name\target.def
```
APP_SUPPORT := YES
```

proj\application\Makefile
```
ifeq "$(strip ${APP_SUPPORT})" "YES" 
LOCAL_MODULE_DEPENDS += application/app
endif
```

proj\application\app\Makefile
```
include ${SOFT_WORKDIR}/env/compilation/mmi_compilevars.mk

LOCAL_NAME := application/app

SRC_DIR := .

VPATH := src1 src2 src3
C_SRC := src1.c src2.c src3.c

LOCAL_ADD_INCLUDE := application/app/inc1 application/app/inc2 application/app/inc3

include ${SOFT_WORKDIR}/env/compilation/compilerules.mk
```

```
git status | grep Makefile
git status application | grep Makefile
```

```
ctmake application/app CT_TARGET=target CT_RELEASE=debug
```

```
include ${SOFT_WORKDIR}/env/compilation/mmi_compilevars.mk

LOCAL_NAME := application/app

SRC_DIR := .

APP_PLATFORM_PREFIX := platform_
APP_PLATFORM := apple
APP_PLATFORM_DIR := $(APP_PLATFORM_PREFIX)$(APP_PLATFORM)

APP_SRC_FILES := $(shell find . -name "*.c" -or -name "*.h")
$(info $(APP_SRC_FILES))
APP_PLATFORM_SRC_FILES := $(filter ./$(APP_PLATFORM_DIR)%,$(APP_SRC_FILES))
$(info $(APP_PLATFORM_SRC_FILES))
APP_SRC_FILES := $(filter-out ./$(APP_PLATFORM_PREFIX)%,$(APP_SRC_FILES))
$(info $(APP_SRC_FILES))
APP_SRC_FILES := $(APP_SRC_FILES) $(APP_PLATFORM_SRC_FILES)
$(info $(APP_SRC_FILES))
APP_SRC_DIRS := $(sort $(dir $(APP_SRC_FILES)))
$(info $(APP_SRC_DIRS))

C_SRC := $(notdir $(filter %c,$(APP_SRC_FILES)))
$(info $(C_SRC))
VPATH := $(APP_SRC_DIRS)
$(info $(VPATH))

LOCAL_ADD_INCLUDE := $(addprefix $(LOCAL_NAME)/,$(APP_SRC_DIRS))
$(error $(LOCAL_ADD_INCLUDE))

include ${SOFT_WORKDIR}/env/compilation/compilerules.mk
```