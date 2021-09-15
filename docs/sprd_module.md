1. MS_Code/make/app1/app1.mk

2. vi MS_Code/SC6530.modules
```
ifeq ($(strip $(APP1_SUPPORT)),TRUE)
COMPLIBS_DBG    +=  app1
endif
```

2. vi MS_Code/make/simulator/simulator.mk
```
ifeq ($(strip $(APP1_SUPPORT)), TRUE)
dsp_modules_cus += app1
endif
```

```
MS_Code>make p={project} m={app}

MS_Code>make\make_cmd\make -r -R MAKESHELL=CMD p={project} m={app}

project_{project}.mk
app custom_drv chip_drv gui mmk lcd ucom ubot upcc usbservice version aud_dev au
d_config scp scm atc refbase dc sim mmk_sys caf_templete utility utility_perform
ance !resource !nv_parameter dv_cfg !wre_preload char_lib main bt_pal !nor_fdl a
utotest_device

Setup Directories For LIBs, OBJs LOGs and temp files
run...

Successfully! Generate c file at: MS_Code\build\{project}_builddir\tmp\ui_special_effect_table.c.

Building library app \( refer to build/{project}_builddir/log/app.log for detail \), Please Wait ...
Directory: build/{project}_builddir/obj/app create OK!
Directory: build/{project}_builddir/dep/app create OK!
make[1]: Entering directory `MS_Code'
make/pclint/pclint.mk:15: make/app/app.mk: No such file or directory
make[1]: *** No rule to make target `make/app/app.mk'.
make[1]: Failed to remake makefile `make/app/app.mk'.
component = app
make[1]: Leaving directory `MS_Code'
make: *** [app.a] Error 2
```

touch make/app/app.mk

```
MS_Code>make p={project} m=app

MS_Code>make\make_cmd\make -r -R MAKESHELL=CMD p={project} m=app

project_{project}.mk
app custom_drv chip_drv gui mmk lcd ucom ubot upcc usbservice version aud_dev au
d_config scp scm atc refbase dc sim mmk_sys caf_templete utility utility_perform
ance !resource !nv_parameter dv_cfg !wre_preload char_lib main bt_pal !nor_fdl a
utotest_device

Setup Directories For LIBs, OBJs LOGs and temp files
run...

Successfully! Generate c file at: MS_Code\build\{project}_builddir\tmp\ui_special_effect_table.c.

Building library app \( refer to build/{project}_builddir/log/app.log for detail \), Please Wait ...
make[1]: Entering directory `MS_Code'
component = app
make[1]: Leaving directory `MS_Code'
Fatal error: L3904U: Could not open via file './objectlist.txt'.
make[1]: *** [app.a] Error 1

app library build finished

******************************** {project} Error list ********************************

[app] Fatal error: L3904U: Could not open via file './objectlist.txt'.
```

vi make/app/app.mk

```
ifeq ($(strip $(APP_SUPPORT)),TRUE)

APPSRCPATH  +=  MS_MMI/source/mmi_app/app/app/m1
APPSRCPATH  +=  MS_MMI/source/mmi_app/app/app/inc

MSRCPATH   += $(APPSRCPATH)
MINCPATH   += $(APPSRCPATH)

SOURCES   += $(foreach dir, ${APPSRCPATH}, $(notdir $(wildcard ${dir}/*.c)))
endif
```

```
Compiling Source File MS_MMI/source/mmi_app/app/app/m1/f1.c...
"MS_MMI/source/mmi_app/app/app/inc/in1.h", line 10: Error:  #5:
cannot open source input file "guiform.h": No such file or directory
  #include "guiform.h"
                      ^
MS_MMI/source/mmi_app/app/app/m1/f1.c: 0 warnings, 1 error

...

make[1]: Target `app.a' not remade because of errors.

app library build finished

app Time consuming: 15 seconds!

******************************** {project} Error list ********************************

[app] "MS_MMI/source/mmi_app/app/app/inc/in1.h", line 10: Error:  #5: cannot open source input file "guiform.h": No such file or directory
[app] "MS_MMI/source/mmi_app/app/app/inc/in1.h", line 13: Error:  #5: cannot open source input file "mmi_position.h": No such file or directory
[app] "MS_MMI/source/mmi_app/app/app/inc/in1.h", line 14: Error:  #5: cannot open source input file "mmiset_export.h": No such file or directory
[app] "MS_MMI/source/mmi_gui/include/guiprgbox.h", line 48: Error:  #5: cannot open source input file "mmk_type.h": No such file or directory
[app] "MS_MMI/source/mmi_kernel/include/mmk_type.h", line 46: Error:  #5: cannot open source input file "caf.h": No such file or directory
[app] "chip_drv/export/inc/sio.h", line 29: Error:  #5: cannot open sourceinput file "chip.h": No such file or directory
[app] "MS_MMI/source/mmi_app/app/setting/h/mmiset_export.h", line 24: Error:  #5: cannot open source input file "mn_type.h": No such file or directory
[app] "MS_MMI/source/mmi_app/app/setting/h/mmiset_export.h", line 26: Error:  #5: cannot open source input file "mmiparse_export.h": No such file or directory
[app] "MS_MMI/source/mmi_app/app/parse/h/mmiparse_export.h", line 20: Error:  #5: cannot open source input file "mmi_common.h": No such file or directory
[app] "MS_MMI/source/mmi_app/common/h/mmi_common.h", line 24: Error:  #5: cannot open source input file "mmi_modu_main.h": No such file or directory
[app] "MS_MMI/source/mmi_gui/include/guimenu.h", line 382: Error:  #5: cannot open source input file "mmi_link.h": No such file or directory
[app] "MS_MMI/source/mmi_app/common/h/mmi_ring.h", line 17: Error:  #5: cannot open source input file "mmisrvaud_api.h": No such file or directory
[app] "MS_MMI/source/mmi_service/source/mmisrvapi/h/mmisrvaud_api.h", line18: Error:  #5: cannot open source input file "mmisrv.h": No such file or directory
[app] "MS_MMI/source/mmi_service/export/mmisrv.h", line 21: Error:  #5: cannot open source input file "mmisrv_debug.h": No such file or directory
[app] "MS_MMI/source/mmi_app/kernel/h/mmi_module.h", line 73: Error:  #5: cannot open source input file "mmi_res_prj_def.h": No such file or directory
[app] "Base/l4/export/inc/simat_data_object.h", line 79: Error:  #5: cannot open source input file "sim_type.h": No such file or directory
[app] "Base/l4/export/inc/simat_api.h", line 31: Error:  #5: cannot open source input file "state_machine.h": No such file or directory
[app] "MS_MMI/source/mmi_app/common/h/mmi_common.h", line 32: Error:  #5: cannot open source input file "mmipb_adapt.h": No such file or directory
...
```

find ../.. -iname "mmisrvaud_api.h"

vi make/app/app.mk

```
ifeq ($(strip $(APP_SUPPORT)),TRUE)
MINCPATH += MS_MMI/source/mmi_gui/include
MINCPATH += MS_MMI/source/mmi_kernel/include 

APPSRCPATH  +=  MS_MMI/source/mmi_app/app/app/m1
APPSRCPATH  +=  MS_MMI/source/mmi_app/app/app/inc

MSRCPATH   += $(APPSRCPATH)
MINCPATH   += $(APPSRCPATH)

SOURCES   += $(foreach dir, ${APPSRCPATH}, $(notdir $(wildcard ${dir}/*.c)))
endif
```
