$ cargo run -vv
       Dirty code-gen v0.1.0 (D:\playground\rust\code-gen): the file `src\main.rs` has changed (13354190695.956299700s, 17s after last build at 13354190678.498598200s)
   Compiling code-gen v0.1.0 (D:\playground\rust\code-gen)
     Running `
        set CARGO='\\?\C:\Users\Admin\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe'&& 
        set CARGO_BIN_NAME=code-gen&& 
        set CARGO_CRATE_NAME=code_gen&& 
        set CARGO_MANIFEST_DIR='D:\playground\rust\code-gen'&& 
        set CARGO_PKG_AUTHORS=''&& 
        set CARGO_PKG_DESCRIPTION=''&& 
        set CARGO_PKG_HOMEPAGE=''&& 
        set CARGO_PKG_LICENSE=''&& 
        set CARGO_PKG_LICENSE_FILE=''&& 
        set CARGO_PKG_NAME=code-gen&& 
        set CARGO_PKG_README=''&& 
        set CARGO_PKG_REPOSITORY=''&& 
        set CARGO_PKG_RUST_VERSION=''&& 
        set CARGO_PKG_VERSION=0.1.0&& 
        set CARGO_PKG_VERSION_MAJOR=0&& 
        set CARGO_PKG_VERSION_MINOR=1&& 
        set CARGO_PKG_VERSION_PATCH=0&& 
        set CARGO_PKG_VERSION_PRE=''&& 
        set CARGO_PRIMARY_PACKAGE=1&& 
        set CARGO_RUSTC_CURRENT_DIR='D:\playground\rust\code-gen'&& 
        set OUT_DIR='D:\playground\rust\code-gen\target\debug\build\code-gen-9d30d1636d8f61ac\out'&& 
        set PATH='
            D:\playground\rust\code-gen\target\debug\deps;
            C:\Users\Admin\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin;
            C:\Users\Admin\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin;
            C:\Users\Admin\bin;
            C:\Program Files\Git\mingw64\bin;
            C:\Program Files\Git\usr\local\bin;
            C:\Program Files\Git\usr\bin;
            C:\Program Files\Git\usr\bin;
            C:\Program Files\Git\mingw64\bin;
            C:\Program Files\Git\usr\bin;
            C:\Users\Admin\bin;
            C:\Program Files (x86)\VMware\VMware Player\bin;
            C:\Windows\system32;
            C:\Windows;
            C:\Windows\System32\Wbem;
            C:\Windows\System32\WindowsPowerShell\v1.0;
            C:\Windows\System32\OpenSSH;
            C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;
            C:\Program Files\Puppet Labs\Puppet\bin;
            C:\Program Files\Git\cmd;
            C:\Program Files\NVIDIA Corporation\NVIDIA NvDLISR;
            C:\Program Files\Go\bin;
            C:\Strawberry\c\bin;
            C:\Strawberry\perl\site\bin;
            C:\Strawberry\perl\bin;
            C:\Users\Admin\.cargo\bin;
            C:\Users\Admin\AppData\Local\Microsoft\WindowsApps;
            C:\Users\Admin\AppData\Local\Programs\Microsoft VS Code\bin;
            C:\Users\Admin\go\bin;
            D:\tools\capnproto-c++-win32-1.0.2\capnproto-tools-win32-1.0.2;
            C:\Program Files\qemu;
            C:\Program Files\Git\usr\bin\vendor_perl;
            C:\Program Files\Git\usr\bin\core_perl'&& 
            
            rustc --crate-name code_gen --edition=2021 'src\main.rs' 
            --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat 
            --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=d2eeaa2e3438650a 
            --out-dir 'D:\playground\rust\code-gen\target\debug\deps' 
            -C 'incremental=D:\playground\rust\code-gen\target\debug\incremental' 
            -L 'dependency=D:\playground\rust\code-gen\target\debug\deps'`

    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `
        set CARGO='\\?\C:\Users\Admin\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe'&& 
        set CARGO_MANIFEST_DIR='D:\playground\rust\code-gen'&& 
        set CARGO_PKG_AUTHORS=''&& 
        set CARGO_PKG_DESCRIPTION=''&& 
        set CARGO_PKG_HOMEPAGE=''&& 
        set CARGO_PKG_LICENSE=''&& 
        set CARGO_PKG_LICENSE_FILE=''&& 
        set CARGO_PKG_NAME=code-gen&& 
        set CARGO_PKG_README=''&& 
        set CARGO_PKG_REPOSITORY=''&& 
        set CARGO_PKG_RUST_VERSION=''&& 
        set CARGO_PKG_VERSION=0.1.0&& 
        set CARGO_PKG_VERSION_MAJOR=0&& 
        set CARGO_PKG_VERSION_MINOR=1&& 
        set CARGO_PKG_VERSION_PATCH=0&& 
        set CARGO_PKG_VERSION_PRE=''&& 
        set OUT_DIR='D:\playground\rust\code-gen\target\debug\build\code-gen-9d30d1636d8f61ac\out'&& 
        set PATH='
            D:\playground\rust\code-gen\target\debug\deps;
            D:\playground\rust\code-gen\target\debug;
            C:\Users\Admin\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib;
            C:\Users\Admin\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin;
            C:\Users\Admin\bin;
            C:\Program Files\Git\mingw64\bin;
            C:\Program Files\Git\usr\local\bin;
            C:\Program Files\Git\usr\bin;
            C:\Program Files\Git\usr\bin;
            C:\Program Files\Git\mingw64\bin;
            C:\Program Files\Git\usr\bin;
            C:\Users\Admin\bin;
            C:\Program Files (x86)\VMware\VMware Player\bin;
            C:\Windows\system32;
            C:\Windows;
            C:\Windows\System32\Wbem;
            C:\Windows\System32\WindowsPowerShell\v1.0;
            C:\Windows\System32\OpenSSH;
            C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;
            C:\Program Files\Puppet Labs\Puppet\bin;
            C:\Program Files\Git\cmd;
            C:\Program Files\NVIDIA Corporation\NVIDIA NvDLISR;
            C:\Program Files\Go\bin;
            C:\Strawberry\c\bin;
            C:\Strawberry\perl\site\bin;
            C:\Strawberry\perl\bin;
            C:\Users\Admin\.cargo\bin;
            C:\Users\Admin\AppData\Local\Microsoft\WindowsApps;
            C:\Users\Admin\AppData\Local\Programs\Microsoft VS Code\bin;
            C:\Users\Admin\go\bin;
            D:\tools\capnproto-c++-win32-1.0.2\capnproto-tools-win32-1.0.2;
            C:\Program Files\qemu;
            C:\Program Files\Git\usr\bin\vendor_perl;
            C:\Program Files\Git\usr\bin\core_perl'&& 
            
            target\debug\code-gen.exe`
Hello, world!