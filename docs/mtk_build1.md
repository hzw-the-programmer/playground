make/{customer}_GPRS.mak
APP_SUPPORT = SRC # LIB

make/Option.mak
ifdef APP_SUPPORT
	COM_DEFS += __APP__
	COMPLIST += app # for src
	ifeq ($(strip $(APP_SUPPORT)),LIB)
		COM_DEFS += __APP_LIB__
		COMPOBJS += vendor\app\app.a # for target
  	endif
endif

make/modisConfig.mak
ifeq ($(strip $(APP_SUPPORT)),LIB)
	MODIS_EN_OBJS += vendor\app\app.lib # for modis
endif

# for COMPLIST src code
make/app/app.mak
ifeq ($(strip $(APP_SUPPORT)),LIB)
APPDIR := vendor/app
APPINCLUDE := -I $(APPDIR)
else
APPDIR := app
include make/app/app_src.mak
endif

INC_DIR += ${APPINCLUDE} \
           plutommi/Framework/GUI/GUI_INC \
           plutommi/Framework/GDI/GDIINC \
           plutommi/Framework/Interface \
           plutommi/mmi/Resource/Inc \
           plutommi/Service/PhbSrv \
           plutommi/Service/SmsSrv \
           interface/fs \
           kal/include \
           plutommi/Framework/LangModule/LangModuleInc

SRC_FILES = $(foreach dir, ${APPINCLUDE}, $(wildcard ${dir}/*.c))
SRC_LIST += ${SRC_FILES}

make/app/app_src.mak
APPINCLUDE += -I $(APPDIR) \
		   += -I $(APPDIR)/src

plutommi/mmi/Inc/mmi_res_range_def.h
#ifdef __APP__
#define APP_BASE ((U16) GET_RESOURCE_BASE(APP))
#define APP_MAX  ((U16) GET_RESOURCE_MAX(APP))
#ifdef __APP_LIB__
RESOURCE_BASE_TABLE_ITEM_PATH(APP, "..\\vendor\\app\\res\\")
#else
RESOURCE_BASE_TABLE_ITEM_PATH(APP, "..\\app\\res\\")
#endif
#endif

notes
```
build/SAGETEL60A_6464_11B/log/infomake.log
gen_modis
build/SAGETEL60A_6464_11B/log/infomake_MoDIS.log
MODIS_EN_OBJS
MoDIS_VC9/createMoDIS.pl

MoDIS_VC9/MoDIS/MoDIS.vcproj:                           RelativePath="..\..\vendor\app\app.lib"
```
