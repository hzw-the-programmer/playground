fn main() {
    println!("cargo:rerun-if-changed=c/foo.c");
    println!("cargo:rerun-if-changed=c/bar.c");
    cc::Build::new()
        .file("c/foo.c")
        .file("c/bar.c")
        .compile("foo");
}

/*
process didn't exit successfully: `D:\playground\rust\cc-test\target\debug\build\cc-test-fc897c01ddde768f\build-script-build` (exit code: 1)
--- stdout
TARGET = Some("x86_64-pc-windows-msvc")
OPT_LEVEL = Some("0")
HOST = Some("x86_64-pc-windows-msvc")
cargo:rerun-if-env-changed=CC_x86_64-pc-windows-msvc
CC_x86_64-pc-windows-msvc = None
cargo:rerun-if-env-changed=CC_x86_64_pc_windows_msvc
CC_x86_64_pc_windows_msvc = None
cargo:rerun-if-env-changed=HOST_CC
HOST_CC = None
cargo:rerun-if-env-changed=CC
CC = None
cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
CRATE_CC_NO_DEFAULTS = None
CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")
DEBUG = Some("true")
cargo:rerun-if-env-changed=CFLAGS_x86_64-pc-windows-msvc
CFLAGS_x86_64-pc-windows-msvc = None
cargo:rerun-if-env-changed=CFLAGS_x86_64_pc_windows_msvc
CFLAGS_x86_64_pc_windows_msvc = None
cargo:rerun-if-env-changed=HOST_CFLAGS
HOST_CFLAGS = None
cargo:rerun-if-env-changed=CFLAGS
CFLAGS = None
cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
foo.c
c1: fatal error C1083: 无法打开源文件: “foo.c”: No such file or directory

--- stderr


error occurred: Command "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.35.32215\\bin\\HostX64\\x64\\cl.exe" "-nologo" "-MD" "-Z7" "-Brepro" "-W4" "-FoD:\\playground\\rust\\cc-test\\target\\debug\\build\\cc-test-4ef704f86d6997da\\out\\d1fba762150c532c-foo.o" "-c" "foo.c" with args "cl.exe" did not execute successfully (status code exit code: 2).
*/

/*
  process didn't exit successfully: `D:\playground\rust\cc-test\target\debug\build\cc-test-fc897c01ddde768f\build-script-build` (exit code: 1)
  --- stdout
  TARGET = Some("x86_64-pc-windows-msvc")
  OPT_LEVEL = Some("0")
  HOST = Some("x86_64-pc-windows-msvc")
  cargo:rerun-if-env-changed=CC_x86_64-pc-windows-msvc
  CC_x86_64-pc-windows-msvc = None
  cargo:rerun-if-env-changed=CC_x86_64_pc_windows_msvc
  CC_x86_64_pc_windows_msvc = None
  cargo:rerun-if-env-changed=HOST_CC
  HOST_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")
  DEBUG = Some("true")
  cargo:rerun-if-env-changed=CFLAGS_x86_64-pc-windows-msvc
  CFLAGS_x86_64-pc-windows-msvc = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_pc_windows_msvc
  CFLAGS_x86_64_pc_windows_msvc = None
  cargo:rerun-if-env-changed=HOST_CFLAGS
  HOST_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  foo.c
  foo.c(1): warning C4206: 使用了非标准扩展: 翻译单元为空
  bar.c
  c1: fatal error C1083: 无法打开源文件: “bar.c”: No such file or directory

  --- stderr


  error occurred: Command "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.35.32215\\bin\\HostX64\\x64\\cl.exe" "-nologo" "-MD" "-Z7" "-Brepro" "-W4" "-FoD:\\playground\\rust\\cc-test\\target\\debug\\build\\cc-test-4ef704f86d6997da\\out\\d1fba762150c532c-bar.o" "-c" "bar.c" with args "cl.exe" did not execute successfully (status code exit code: 2).
*/
