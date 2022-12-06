https://fileinfo.com/extension/idb
What is an IDB file?
Intermediate file created by a Visual Studio program, such as Visual C++, during the debugging process; saves the compiler's state and is used for minimal program rebuilds and incremental compilations.

https://stackoverflow.com/questions/3899573/what-is-a-pdb-file
PDB is an abbreviation for Program-Debug Data Base. As the name suggests, it is a repository (persistent storage such as databases) to maintain information required to run your program in debug mode. It contains several vital information required for code debugging e.g. at what points you have put break points where you expect the debugger to break in Visual Studio (VS).

https://learn.microsoft.com/en-us/cpp/build/reference/dot-ilk-files-as-linker-input?view=msvc-170
When linking incrementally, LINK updates the .ilk status file that it created during the first incremental link. This file has the same base name as the target EXE or DLL file, and it has the extension .ilk. During subsequent incremental links, LINK updates the .ilk file. If the .ilk file is missing, LINK performs a full link and creates a new .ilk file. If the .ilk file is unusable, LINK performs a non-incremental link.
