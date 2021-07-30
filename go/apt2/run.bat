@echo off
SET prj=
SET app=%prj%\
SET arm=%prj%\
SET vc=%prj%\
SET first_commit_file=%prj%\
SET provide=provide.txt
SET exclude=exclude.txt
SET libout=
SET provideout=

apppkg.exe ^
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
