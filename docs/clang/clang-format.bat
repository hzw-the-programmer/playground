@rem @echo off
@SET PATH=C:\Program Files\LLVM\bin;%PATH%
@for /r %%f in (*.c,*.h) do clang-format.exe -i %%f
@pause