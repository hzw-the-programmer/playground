rustup default nightly

rustc --print sysroot
${RUSTUP_HOME}\toolchains\nightly-x86_64-pc-windows-msvc

rustup component add rust-src
${RUSTUP_HOME}\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src

rustup component add llvm-tools
${RUSTUP_HOME}\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin

rustup default stable
