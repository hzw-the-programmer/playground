project_project_240X320BAR_64MB.mk
```
# *************************************************************************
# Feature Options (For User)
...
DEMOAPP_SUPPORT = TRUE

#****************************************************************************************#
#  Segment: SPRD MACRO
#  Description: Get all sprd macro list
#****************************************************************************************#
```

make/demoapp/demoapp.mk
```
ifeq ($(strip $(DEMOAPP_SUPPORT)), TRUE)
include make/app_main/app_main.mk

MINCPATH += demoapp/h
MSRCPATH = demoapp/c
SOURCES = demoapp.c
SOURCES += log.c
endif
```

PROJECT.modules
```
#****************************************************************************************#
#  Segment: COMPLIBS
#  Description: The Following Libs Will Be Compiled By Default (Using Make PROJECT update or New)
#****************************************************************************************#
...
ifeq ($(strip $(DEMOAPP_SUPPORT)),TRUE)
COMPLIBS_DBG += demoapp
endif

#****************************************************************************************#
#  Segment: LINKLIBLIST
#  Description: Libs Need By The Final Target
#               Custom Can Add Lib or Obj files
#****************************************************************************************#
```

MS_MMI_Main/source/resource/mmi_res_prj_def.h
```
#if defined(DEMOAPP_SUPPORT)
RES_ADD_MODULE(MMI_MODULE_DEMOAPP,"\\demoapp\\demoapp_mdu_def.h")
#endif
```

make/resource_main/resource_header.mk
```
ifeq ($(strip $(DEMOAPP_SUPPORT)), TRUE)
SOURCES	 += demoapp_mdu_def.h
SRCPATH += demoapp/h
endif
```

make p=project_240X320BAR_48MB_CAT1 m=resource_main

MS_MMI_Main/source/mmi_app/kernel/h/mmk_regapp.def
```
#if defined(DEMOAPP_SUPPORT)
REG_APP(SOCKET_SIG_BEGIN, SOCKET_SIG_END, &g_demoapp)
#endif
```

MS_MMI_Main/source/mmi_app/kernel/h/mmk_ext_app.h
```
#if defined(DEMOAPP_SUPPORT)
    extern MMI_APPLICATION_T g_demoapp;
#endif
```

MS_MMI_Main/source/mmi_app/kernel/c/mmimain.c
```
APP_Init() {
    ...
#if defined(DEMOAPP_SUPPORT)
    demoapp_init();
#endif
}
```

make/simulator_main/simulator_main.mk
```
# *************************************************************************
# Config Start
# *************************************************************************

ifeq ($(strip $(DEMOAPP_SUPPORT)), TRUE)
dsp_modules_cus += demoapp
endif

# *************************************************************************
# Config End
# *************************************************************************
```