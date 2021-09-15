```
vi make\option.mak
COMPLIST += app

make {custom} gprs new

...
Generate vcard information
Generate vendorapp information
Error: .lis Failed! Please check .\build\{custom}\log\module_info\_module_info.log
make[1]: *** [.\build\{custom}\module\operac\operac.def] Error 1
make[1]: *** Waiting for unfinished jobs....
make: *** [genmoduleinfo] Error 2
new

vi .\build\{custom}\log\module_info\_module_info.log

make\comp.mak:154: make\\.mak: No such file or directory
Module  is in DEFALUT mode
make[2]: *** No rule to make target `make\\.mak'.
make[2]: Failed to remake makefile `make\\.mak'.
Generating  information is done.
```

```
mkdir -p make\app\app.mak
make {custom} gprs new

2021/09/09 16:46:01
 LOG: .\build\{custom}\log\udx.log
make[1]: *** [app.lib] Error 1
make[1]: *** Waiting for unfinished jobs....
make[1]: Leaving directory `D:/work/project'

make: *** [xgc_all_libs_2] Error 1
new

Module app is in DEFALUT mode
make[2]: Entering directory `D:/work/project'
CFLAGS = --cpu ARM7EJ-S --littleend -O3 --remove_unneeded_entities -D__RVCT__ -JD:\RVCT\RVCT\Data\3.1\569\include\windows --fpmode=ieee_fixed --split_sections --diag_suppress 1,1295,1296,2548 --dwarf2 -D__SERIAL_FLASH_EN__ -D__SERIAL_FLASH_SUPPORT__ --bss_threshold=0
CPLUSFLAGS = --cpp --cpu ARM7EJ-S --littleend -O3 --remove_unneeded_entities -D__RVCT__ -JD:\RVCT\RVCT\Data\3.1\569\include\windows --fpmode=ieee_fixed --split_sections --diag_suppress 1,1295,1296,2548 --dwarf2 --bss_threshold=0
AFLAGS = --debug --littleend --cpu ARM7EJ-S --apcs /interwork -16
ADEFS = -pd "MT6260 SETL {TRUE}" -pd "__HW_DIVIDER__ SETL {TRUE}" -pd "MT6260_S00 SETL {TRUE}" -pd "__SV5_ENABLED__ SETL {TRUE}" -pd "__EVENT_BASED_TIMER__ SETL {TRUE}" -pd "__PRODUCTION_RELEASE__ SETL {TRUE}" -pd "KAL_ON_NUCLEUS SETL {TRUE}" -pd "__CHIP_VERSION_CHECK__ SETL {TRUE}" -pd "__ZIMAGE_SUPPORT__ SETL {TRUE}" -pd "ESAL_AR_STK_FPU_SUPPORT SETL {FALSE}" -pd "__ARM_FPUV2__ SETL {FALSE}" -pd "__SERIAL_FLASH_EN__ SETL {TRUE}" -pd "__SERIAL_FLASH_SUPPORT__ SETL {TRUE}"
Cannot open .\build\{custom}\gprs\{platform}\app\app.via: No such file or directory at .\tools\sortobj.pl line 60.
make[2]: *** [build/{custom}/gprs/{platform}/lib/app.lib] Error 2
make[2]: Leaving directory `D:/work/project}'
```

```
make {custom} gprs new

...
Generate config information
Error: config.lis Failed! Please check .\build\{custom}\log\module_info\config_module_info.log
make[2]: *** [.\build\{custom}\module\config\config.def] Error 1
make[1]: *** [genmoduleinfo] Error 2
make: *** [.\build\{custom}\gprs\{platform}\pregen_dep\sys_mem_gen.det] Error 2
new
```
