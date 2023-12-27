Configuration -> Manage Configurations
```
$ cat CMakeSettings.json
{
  "configurations": [
    {
      "name": "x64-Debug",
      "generator": "Ninja",
      "configurationType": "Debug",
      "inheritEnvironments": [ "msvc_x64_x64" ],
      "buildRoot": "${projectDir}\\out\\build\\${name}",
      "installRoot": "${projectDir}\\out\\install\\${name}",
      "cmakeCommandArgs": "",
      "buildCommandArgs": "",
      "ctestCommandArgs": ""
    }
  ]
}
```

```
$ cat CMakeSettings.json
{
  "configurations": [
    {
      "name": "x64-Debug",
      "generator": "Ninja",
      "configurationType": "Debug",
      "inheritEnvironments": [ "msvc_x64_x64" ],
      "buildRoot": "${projectDir}\\out\\build\\${name}",
      "installRoot": "${projectDir}\\out\\install\\${name}",
      "cmakeCommandArgs": "",
      "buildCommandArgs": "",
      "ctestCommandArgs": "",
      "variables": [
        {
          "name": "GEN_FILES",
          "value": "True",
          "type": "BOOL"
        }
      ]
    }
  ]
}
```

```
1> 已为配置“x64-Debug”启动 CMake 生成。
1> 命令行: "C:\Windows\system32\cmd.exe" /c "%SYSTEMROOT%\System32\chcp.com 65001 >NUL && "C:\PROGRAM FILES\MICROSOFT VISUAL STUDIO\2022\COMMUNITY\COMMON7\IDE\COMMONEXTENSIONS\MICROSOFT\CMAKE\CMake\bin\cmake.exe"  -G "Ninja"  -DCMAKE_BUILD_TYPE:STRING="Debug" -DCMAKE_INSTALL_PREFIX:PATH="F:\open_source\open_source1\github\Mbed-TLS\mbedtls\out\install\x64-Debug" -DGEN_FILES:BOOL="True" -DCMAKE_C_COMPILER:FILEPATH="C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.35.32215/bin/Hostx64/x64/cl.exe" -DCMAKE_CXX_COMPILER:FILEPATH="C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.35.32215/bin/Hostx64/x64/cl.exe"  -DCMAKE_MAKE_PROGRAM="C:\PROGRAM FILES\MICROSOFT VISUAL STUDIO\2022\COMMUNITY\COMMON7\IDE\COMMONEXTENSIONS\MICROSOFT\CMAKE\Ninja\ninja.exe" "F:\open_source\open_source1\github\Mbed-TLS\mbedtls" 2>&1"
1> 工作目录: F:\open_source\open_source1\github\Mbed-TLS\mbedtls\out\build\x64-Debug
1> [CMake] -- Configuring done
1> [CMake] -- Generating done
1> [CMake] -- Build files have been written to: F:/open_source/open_source1/github/Mbed-TLS/mbedtls/out/build/x64-Debug
1> 已提取 CMake 变量。
1> 已提取源文件和标头。
1> 已提取代码模型。
1> 已提取工具链配置。
1> 已提取包含路径。
1> CMake 生成完毕。
```
