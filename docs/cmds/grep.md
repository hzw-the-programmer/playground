grep "static_assert" --include="*.cc" -r .
grep "static_assert" --exclude="*.cc" -r .

find make -maxdepth 1 -type f | xargs grep "cpp"
```
make/Comp.mak:.SUFFIXES: .obj .c .s .cpp .arm .ltp .det
make/Comp.mak:CPPSRCS := $(filter %.cpp, $(SRCLIST0))
make/Comp.mak:define target_compile_cpp_obj
make/Comp.mak:define target_scan_cpp_det
```

find make -maxdepth 1 -type f -exec grep "cpp" '{'} \;
```
.SUFFIXES: .obj .c .s .cpp .arm .ltp .det
CPPSRCS := $(filter %.cpp, $(SRCLIST0))
define target_compile_cpp_obj
define target_scan_cpp_det
```
find make -maxdepth 1 -type f -exec grep "cpp" '{'} +
```
make/Comp.mak:.SUFFIXES: .obj .c .s .cpp .arm .ltp .det
make/Comp.mak:CPPSRCS := $(filter %.cpp, $(SRCLIST0))
make/Comp.mak:define target_compile_cpp_obj
make/Comp.mak:define target_scan_cpp_det
```

Run file on every file in or below the current directory.
find . -type f -exec file '{}' \;
In many cases, one might prefer the `-exec ... +` or better the
`-execdir ... +` syntax for performance and security reasons.

find . -type f -exec file '{}' +
-exec command {} +
but the command line is
built by appending each selected file name at the end; the
total number of invocations of the command will be much
less than the number of matched files.  The command line
is built in much the same way that xargs builds its
command lines.  Only one instance of `{}' is allowed
within the command, and it must appear at the end,
immediately before the `+'; it needs to be escaped (with a
`\') or quoted to protect it from interpretation by the
shell.

### --exclude
grep "word" -r --color --exclude={*.obj,*.pdb,*.sdf,*.pch,*.idb,*.lib} .

### grep "word" -rl .
### grep "word" -rn .