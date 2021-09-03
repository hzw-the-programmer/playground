make\project_GPRS.mak

```
make project gprs gen_modis
make project gprs resgen
make project gprs new_modis
```

MoDIS_VC9\MoDIS.sln

```
make project gprs r app
make project gprs c app
```

change header file name then
make project gprs r app
no rule ... xxx.h ...
grep -r "xxx.h" build
build/project/gprs/{platfom}r/app.dep
"make project gprs u app" or delete build/project/gprs/{platfom}r/app.dep
"make project gprs c app" does not work
because "c" only rm build/project/gprs/{platfom}o/*.obj

./build/PLT_PRJ/gprs/PLTr/app.dep
make project gprs r app

grep -r "BrowserAppInterface.c" make
make/plutommi/inet_app/inet_app.mak:SRC_LIST += plutommi\MMI\BrowserApp\Browser\BrowserSrc\BrowserAppInterface.c
make project gprs r inet_app

make {custom} {project} codegen
