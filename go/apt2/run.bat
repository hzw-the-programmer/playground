@echo off
SET prj=
SET app=%prj%\
REM SET arm=%prj%\build\PLT_PRJ\gprs\PLTo\app
REM SET vc=%prj%\MoDIS_VC9\app\Debug
SET arm=%prj%\MS_Code\build\PRJ_builddir\obj\app
SET vc=%prj%\MS_Code\build\PRJ_builddir\win\app\Debug
SET first_commit_file=%prj%\
SET provide=provide\provide.txt
SET exclude=exclude\exclude.txt
SET libout=
SET provideout=

rem for lib.exe
rem SET PATH=C:\Program Files (x86)\Microsoft Visual Studio\VC98\bin;%PATH%
SET PATH=C:\CSDTK\cygwin\crosscompiler\bin;%PATH%
rem for dll
rem SET PATH=C:\Program Files (x86)\Microsoft Visual Studio\Common\MSDev98\Bin;%PATH%
SET PATH=C:\CSDTK\cygwin\bin;%PATH%

bin\apppkg.exe ^
         -prjdir=%prj% ^
         -appdir=%app% ^
         -metainfofn=%app%\ra_feature_def.h ^
         -objdir=%arm% ^
         -mobjdir=%vc% ^
         -firstcf=%first_commit_file% ^
         -libname=bd_im ^
         -providefn=%provide% ^
         -excludefn=%exclude% ^
		 -liboutdir=%libout% ^
		 -provideoutdir=%provideout%
pause
