# config
```
"C:\Windows\system32\cmd.exe" /c
"

%SYSTEMROOT%\System32\chcp.com 65001 >NUL

&&

"C:\PROGRAM FILES\MICROSOFT VISUAL STUDIO\2022\COMMUNITY\COMMON7\IDE\COMMONEXTENSIONS\MICROSOFT\CMAKE\CMake\bin\cmake.exe"
-G "Ninja"
-DCMAKE_C_COMPILER:STRING="cl.exe"
-DCMAKE_CXX_COMPILER:STRING="cl.exe"
-DCMAKE_BUILD_TYPE:STRING="Debug"
-DCMAKE_INSTALL_PREFIX:PATH="D:/open_source/open_source1/github/hzw-the-programmer/playground/c/2022/CMakeSln1/out/install/x64-debug"
-DCMAKE_MAKE_PROGRAM="C:\PROGRAM FILES\MICROSOFT VISUAL STUDIO\2022\COMMUNITY\COMMON7\IDE\COMMONEXTENSIONS\MICROSOFT\CMAKE\Ninja\ninja.exe"
"D:\open_source\open_source1\github\hzw-the-programmer\playground\c\2022\CMakeSln1"
2>&1

"
```

# compile
```
C:\PROGRA~1\MICROS~4\2022\COMMUN~1\VC\Tools\MSVC\1435~1.322\bin\Hostx64\x64\cl.exe
/nologo
-ID:\open_source\open_source1\github\hzw-the-programmer\playground\c\2022\CMakeSln1\CMakePro1\..\include
/DWIN32
/D_WINDOWS
/W3
/MDd
/Ob0
/Od
/RTC1
-ZI
/showIncludes
/FoCMakePro1\CMakeFiles\CMakePro1.dir\CMakePro1.c.obj
/FdCMakePro1\CMakeFiles\CMakePro1.dir\
/FS
-c
D:\open_source\open_source1\github\hzw-the-programmer\playground\c\2022\CMakeSln1\CMakePro1\CMakePro1.c
```

# link
```
cmd.exe /C
"

cd .

&&

"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe"
-E vs_link_exe
--intdir=CMakePro1\CMakeFiles\CMakePro1.dir
--rc=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\rc.exe
--mt=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\mt.exe
--manifests
--
C:\PROGRA~1\MICROS~4\2022\COMMUN~1\VC\Tools\MSVC\1435~1.322\bin\Hostx64\x64\link.exe
/nologo
CMakePro1\CMakeFiles\CMakePro1.dir\CMakePro1.c.obj
/out:CMakePro1\CMakePro1.exe
/implib:CMakePro1\CMakePro1.lib
/pdb:CMakePro1\CMakePro1.pdb
/version:0.0
/machine:x64
/debug
/INCREMENTAL
/subsystem:console
-LIBPATH:D:\open_source\open_source1\github\hzw-the-programmer\playground\c\2022\CMakeSln1\CMakePro1\..\lib
mbedtlsa.lib  kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib

&&

cd .

"
```