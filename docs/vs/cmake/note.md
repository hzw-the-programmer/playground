"C:\Windows\system32\cmd.exe"

/c

"

%SYSTEMROOT%\System32\chcp.com 65001 >NUL

&&

"C:\PROGRAM FILES\MICROSOFT VISUAL STUDIO\2022\COMMUNITY\COMMON7\IDE\COMMONEXTENSIONS\MICROSOFT\CMAKE\CMake\bin\cmake.exe"
-G "Ninja"
-DCMAKE_C_COMPILER:STRING="cl.exe"
-DCMAKE_CXX_COMPILER:STRING="cl.exe"
-DCMAKE_BUILD_TYPE:STRING="Debug"
-DCMAKE_INSTALL_PREFIX:PATH="D:/open_source/open_source1/github/hzw-the-programmer/playground/c/2022/CMakeSln2/out/install/x64-debug"
-DCMAKE_MAKE_PROGRAM="C:\PROGRAM FILES\MICROSOFT VISUAL STUDIO\2022\COMMUNITY\COMMON7\IDE\COMMONEXTENSIONS\MICROSOFT\CMAKE\Ninja\ninja.exe"
"D:\open_source\open_source1\github\hzw-the-programmer\playground\c\2022\CMakeSln2"
2>&1

"