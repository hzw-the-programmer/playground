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
make project gprs u app
"make project gprs c app" does not work
because "c" only rm build/project/gprs/{platfom}o/*.obj
