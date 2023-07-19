Visual Studio 2022 Developer Command Prompt v17.5.3
where cmake
C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe

mkdir build
cd build
cmake ..
cmake -DGEN_FILES=ON ..

C:\Users\Admin\AppData\Local\Programs\Python\Python311\python.exe -m pip install jsonschema
# C:\Users\Admin\AppData\Local\Programs\Python\Python311\Lib\site-packages\jsonschema
# C:\Users\Admin\AppData\Local\Programs\Python\Python311\Scripts\jsonschema.exe
C:\Users\Admin\AppData\Local\Programs\Python\Python311\python.exe -m pip install jinja2
# C:\Users\Admin\AppData\Local\Programs\Python\Python311\Lib\site-packages\jinja2
C:\Users\Admin\AppData\Local\Programs\Python\Python311\python.exe scripts\generate_driver_wrappers.py