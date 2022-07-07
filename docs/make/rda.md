
# environment
CSDTK\cygwin\Cygwin.bat
CSDTK\cygwin\.bashrc   -> CSDTK\cygwin\home\Admin\.bashrc
``alias  work='source ./cygenv.sh'``
CSDTK\cygwin\cygenv.sh -> CSDTK\cygwin\home\Admin\cygenv.sh
``source ${SOFT_WORKDIR}/env/set_env.sh``

// helper script
env\utils\ctmake

# compile
env/compilation/compilerules.mk

FULL_DEPENDENCY_COMPILE

// include
MYINCLUDEDIR

// src
FULL_SRC_OBJECTS

# add a module, named app
```
$ ctmake application/app

*** Error: Nonexistent Module: application/app
```

```
$ mkdir application/app
$ ctmake application/app

+ make -r -j 4 CT_MODULES=application/app
env/compilation/mmi_compilevars.mk:17: *** "!!!! CT_TARGET not valid - TARGET_DIR=target or CT_TARGET= is not supported or not well defined !!!!".  Stop.
```
Makefile
include ${SOFT_WORKDIR}/env/compilation/mmi_compilevars.mk
ifeq "$(wildcard ${TARGET_DIR}/${CT_TARGET}/target.def)" ""
    $(error "!!!! CT_TARGET not valid - TARGET_DIR=${TARGET_DIR} or CT_TARGET=${CT_TARGET} is not supported or not well defined !!!!")
else
# Include the Target definition
include ${TARGET_DIR}/${CT_TARGET}/target.def
endif

``
$ ctmake application/app CT_TARGET=test_target

+ make -r -j 4 CT_MODULES=application/app CT_TARGET=test_target
env/compilation/compilerules.mk:57: *** "!!!! CT_RELEASE= - Not a valid release type !!!!".  Stop.
``
Makefile
include ${SOFT_WORKDIR}/env/compilation/compilerules.mk
VALID_RELEASE_LIST :=release debug calib sa_profile cool_profile custom
SELECTED_RELEASE := $(filter $(VALID_RELEASE_LIST), $(CT_RELEASE))
ifeq "$(SELECTED_RELEASE)" ""
    $(error "!!!! CT_RELEASE=${CT_RELEASE} - Not a valid release type !!!!")
endif

```
$ ctmake application/app CT_TARGET=test_target CT_RELEASE=debug

+ make -r -j 4 CT_MODULES=application/app CT_TARGET=test_target CT_RELEASE=debug

GCC version is 4.4.2
Binutils version is 2.20



LDGEN             8809.ld
LD                bcpu_test_target_debug.elf
CP                Elf binary & map file



MAKE              application
svn: '/cygdrive/e/Code/proj/application' is not a working copy
svn: '/cygdrive/e/Code/proj/application' is not a working copy
REVGEN            applicationp_version.h @ r

PREPARING         libapplication_debug.a
        (added libcoolmmi_debug.a objects)
        (added libmmi_customer_debug.a objects)
        (added libadaptation_debug.a objects)
        (added libthirdpartylibs_debug.a objects)
        (All submodules objects added)
AR                libapplication_debug.a


GEN               stripped (rm syms) elf file stripped_bcpu_elf_file.elf
LDGEN             8809.ld-stage1
LD                test_target_debug.elf-stage1
mips-elf-ld: warning: section `.sys_sram_overlay_2' type changed to PROGBITS
CP                Elf binary & map file



---------------------------------------------------
The first stage of code compression is finished
Starting the second stage ...
---------------------------------------------------


Compress          xcpu_overlay_init_func
Compress          xcpu_overlay_csdapp
Compress          xcpu_overlay_wap
Compress          xcpu_overlay_jmetoc
/cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_wap.bin:     37.5% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_wap.bin.lzma
/cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_csdapp.bin:  37.5% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_csdapp.bin.lzma
/cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_jmetoc.bin:  -inf% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_jmetoc.bin.lzma

/cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_init_func.bin:       45.6% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_init_func.bin.lzma
cksum             xcpu_overlay_wap


cksum             xcpu_overlay_csdapp
cksum             xcpu_overlay_jmetoc



cksum             xcpu_overlay_init_func

Compress          xcpu_overlay_other
Compress          overlay_2_section_1
Compress          overlay_2_section_2

Compress          overlay_2_section_3
/cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_other.bin:   37.5% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/xcpu_overlay_other.bin.lzma
/cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_2_section_1.bin:  37.5% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_2_section_1.bin.lzma


cksum             overlay_2_section_1
cksum             xcpu_overlay_other
/cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_2_section_3.bin:  37.5% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_2_section_3.bin.lzma



cksum             overlay_2_section_3
Compress          overlay_3_section_1
Compress          overlay_3_section_2
/cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_2_section_2.bin:  46.2% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_2_section_2.bin.lzma

/cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_3_section_2.bin:  37.5% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_3_section_2.bin.lzma
Compress          overlay_3_section_3

cksum             overlay_2_section_2
/cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_3_section_1.bin:  42.4% -- replaced with /cygdrive/e/Code/proj/build/test_target/_default_/8809/overlay_3_section_1.bin.lzma

cksum             overlay_3_section_2

cksum             overlay_3_section_1

...

GEN               stripped (rm syms) elf file stripped_bcpu_elf_file.elf
LDGEN             8809.ld
LD                test_target_debug.elf
mips-elf-ld: Flash overflow: there is no more space available in flash memory.
mips-elf-ld: Flash overflow: there is no more space available in flash memory.
make[4]: *** [/cygdrive/e/Code/proj/build/test_target/_default_/8809/test_target_debug.elf] Error 1
make[3]: *** [/cygdrive/e/Code/proj/build/test_target/_default_/8809/test_target_debug.srec] Error 2
make[2]: *** [lod] Error 2
make[1]: *** [bin] Error 2
make: *** [lod] Error 2
```

## add to compile system 
application/Makefile
ifeq ($(findstring application,${GLOBAL_BINARY_LIBS}),)

ifeq "$(strip ${COMPILE_MODE})" "ALL" 

LOCAL_MODULE_DEPENDS := 
LOCAL_MODULE_DEPENDS += application/coolmmi
LOCAL_MODULE_DEPENDS += ${MMI_CUSTOMER}
LOCAL_MODULE_DEPENDS += application/adaptation
LOCAL_MODULE_DEPENDS += application/thirdpartylibs

LOCAL_MODULE_DEPENDS += application/app
endif

ifeq "${CT_MODEM}" "2"
LOCAL_MODULE_DEPENDS += application/diag
endif

else # GLOBAL_BINARY_LIBS

LOCAL_BINARY_LIBS    += application
LOCAL_MODULE_DEPENDS := application

endif # GLOBAL_BINARY_LIBS

```
$ ctmake application/app CT_TARGET=forme_test_8809e2 CT_RELEASE=debug

+ make -r -j 4 CT_MODULES=application/app CT_TARGET=test_target CT_RELEASE=debug

GCC version is 4.4.2
Binutils version is 2.20



LDGEN             8809.ld
LD                bcpu_test_target_debug.elf
CP                Elf binary & map file



MAKE              application
svn: '/cygdrive/e/Code/proj/application' is not a working copy
svn: '/cygdrive/e/Code/proj/application' is not a working copy

MAKE              application/app
make[4]: *** No rule to make target `all'.  Stop.
make[3]: *** [dependencies] Error 2
make[2]: *** [dependencies] Error 2
make[1]: *** [/cygdrive/e/Code/proj/build/test_target/_default_/8809/test_target_debug.srec] Error 2
make: *** [lod] Error 2
```

application/app/Makefile
include ${SOFT_WORKDIR}/env/compilation/mmi_compilevars.mk

include ${SOFT_WORKDIR}/env/compilation/compilerules.mk

```
$ ctmake application/app CT_TARGET=test_target CT_RELEASE=debug

+ make -r -j 4 CT_MODULES=application/app CT_TARGET=test_target CT_RELEASE=debug

GCC version is 4.4.2
Binutils version is 2.20



LDGEN             8809.ld
LD                bcpu_test_target_debug.elf
CP                Elf binary & map file



MAKE              application
svn: '/cygdrive/e/Code/proj/application' is not a working copy
svn: '/cygdrive/e/Code/proj/application' is not a working copy

MAKE              application/app
AR                lib_debug.a

PREPARING         libapplication_debug.a
mips-elf-ar: /cygdrive/e/Code/proj/build/test_target/_default_/application/app/lib/libapp_debug.a: No such file or directory
make[3]: *** [/cygdrive/e/Code/proj/build/test_target/_default_/application/lib/libapplication_debug.a] Error 9
make[2]: *** [dependencies] Error 2
make[1]: *** [/cygdrive/e/Code/proj/build/test_target/_default_/8809/forme_test_target.srec] Error 2
make: *** [lod] Error 2
```
