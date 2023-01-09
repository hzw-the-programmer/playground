C:\Program Files (x86)\Microsoft Visual Studio 9.0\VC\bin\dumpbin.exe

dumpbin /SYMBOLS sha1.obj
https://learn.microsoft.com/en-us/cpp/build/reference/symbols?view=msvc-170

dumpbin /HEADERS sizeof.obj
https://learn.microsoft.com/en-us/cpp/build/reference/headers?view=msvc-170

dumpbin /SECTION:.text sizeof.obj
https://learn.microsoft.com/en-us/cpp/build/reference/section-dumpbin?view=msvc-170

dumpbin /SECTION:.text /RAWDATA sizeof.obj
https://learn.microsoft.com/en-us/cpp/build/reference/rawdata?source=recommendations&view=msvc-170

dumpbin /ARCHIVEMEMBERS AVLib.lib
https://learn.microsoft.com/en-us/cpp/build/reference/archivemembers?view=msvc-170

dumpbin /DEPENDENTS MoDIS.exe
https://learn.microsoft.com/en-us/cpp/build/reference/dependents?view=msvc-170

dumpbin /DIRECTIVES sizeof.obj
dumpbin /DISASM sizeof.obj
dumpbin /EXPORTS MoDIS.exe
dumpbin /EXPORTS AVLib.dll
dumpbin /IMPORTS AVLib.dll
dumpbin /LINKERMEMBER AVLib.lib
