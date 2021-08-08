@echo off
SET prj=
REM SET src=%prj%\MS_Code\MS_MMI\source\mmi_app\app\app
REM SET map=%prj%\MS_Code\build\project_builddir\img\platform_project.map
SET src=%prj%\app
SET map=%prj%\build\PLT_PRJ\PLT_PRJ_PCB01_gprs_PLT_S00.lis

romcalc.exe %src% %map%
pause
