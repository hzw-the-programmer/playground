make\Option.mak
```
ifeq ($(strip $(COMPILER)),RVCT)
  DIR_ARM  = D:\RVCT\RVCT
  DIR_ARM := $(strip $(DIR_ARM))
  ifeq ($(strip $(RVCT_VERSION)),V31)
    DIR_TOOL       =  $(DIR_ARM)\Programs\3.1\569\win_32-pentium
    DIR_ARMLIB     =  $(DIR_ARM)\Data\3.1\569\lib
    DIR_ARMINC     =  $(DIR_ARM)\Data\3.1\569\include\windows
  endif

  DIR_TOOL := $(strip $(DIR_TOOL))
  LINK           =  $(DIR_TOOL)\armlink.exe          # Linker
  ASM            =  $(DIR_TOOL)\armasm.exe           # ARM assembler
  LIB            =  $(DIR_ARM)\Programs\3.1\569\win_32-pentium\armar.exe         # Library tool
  BIN_CREATE     =  $(DIR_TOOL)\fromelf.exe          # Binary tool

  ifeq ($(strip $(COMPILE_MODE)),INST16)
    CC          =  $(DIR_TOOL)\armcc.exe --thumb $(VFP_OPTION_SOFT)     # Thumb Mode(16bits), use tcc
    CC32        =  $(DIR_TOOL)\armcc.exe --arm $(VFP_OPTION)       # ARM Mode(32bits), use armcc
  else
    ifeq ($(strip $(COMPILE_MODE)),INST32)
      CC          =  $(DIR_TOOL)\armcc.exe --arm $(VFP_OPTION)     # ARM Mode(32bits), use armcc
    else
      CC          =  $(DIR_TOOL)\armcc.exe --thumb $(VFP_OPTION_SOFT)   # Default tcc
      CC32        =  $(DIR_TOOL)\armcc.exe --arm $(VFP_OPTION)     # ARM Mode(32bits), use armcc
    endif
  endif
endif
// D:\RVCT\RVCT\Programs\3.1\569\win_32-pentium
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/armar.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/armasm.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/armcc.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/armcpp.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/armlink.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/fromelf.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/tcc.exe
// /d/RVCT/RVCT/Programs/3.1/569/win_32-pentium/tcpp.exe
```

```
FIXPATH        =  .

BUILDDIR       =  $(strip $(FIXPATH))\build
// build
TARGDIR        =  $(strip $(BUILDDIR))\$(strip $(CUSTOMER))
// build\{customer}
PROJDIR        =  $(strip $(TARGDIR))\$(PROJECT)
// build\{customer}\gprs

OBJSDIR        =  $(strip $(PROJDIR))\$(strip $(PLATFORM))o
// build\{customer}\gprs\MT6260o
RULESDIR       =  $(strip $(PROJDIR))\$(strip $(PLATFORM))r
// build\{customer}\gprs\MT6260r
COMPLIBDIR     =  $(strip $(OBJSDIR))\lib
// build\{customer}\gprs\MT6260o\lib
COMPLOGDIR     =  $(strip $(TARGDIR))\log
// build\{customer}\log
```