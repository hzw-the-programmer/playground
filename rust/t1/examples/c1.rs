#[link(name = "mylib", kind = "static")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { add(1, 1) });
}

/*
"C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.35.32215\\bin\\HostX64\\x64\\link.exe"
"/NOLOGO"
"C:\\Users\\Admin\\AppData\\Local\\Temp\\rustcvn2za0\\symbols.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.25q9etgh09im62f9.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.2voj6glu3rvsnah2.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.3chm07wctojpe63u.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.49sornl79ds1984x.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.4vua4o01s8fyn7g8.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.e0aij2jsuccc4b4.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.3w3albfpnjxbwew8.rcgu.o"
"/LIBPATH:D:\\playground\\rust\\t1\\target\\debug\\deps"
"/LIBPATH:C:\\Users\\Admin\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_x86_64_msvc-0.52.0\\lib"
"/LIBPATH:C:\\Users\\Admin\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_x86_64_msvc-0.48.5\\lib"
"/LIBPATH:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd-d67e7b8bedb11cdc.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libpanic_unwind-64b1a07f55304d5e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_demangle-f120462e85f67653.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd_detect-d983e2a0a2a0c17a.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libhashbrown-3ef2a86080fe9062.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_alloc-eb4d69e1a344b307.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunwind-5d40568d59362563.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcfg_if-23ecb7d440e7060a.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liblibc-d852afaab40eda7f.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc-20556e3a8d3a4922.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_core-2ad8a2d023c9ae2e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-0731c9d2f2fdad4e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-8ed27bc4ebec64d5.rlib"
"kernel32.lib"
"advapi32.lib"
"bcrypt.lib"
"kernel32.lib"
"ntdll.lib"
"userenv.lib"
"ws2_32.lib"
"kernel32.lib"
"ws2_32.lib"
"kernel32.lib"
"ntdll.lib"
"kernel32.lib"
"msvcrt.lib"
"/NXCOMPAT"
"/LIBPATH:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
"/OUT:D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.exe"
"/OPT:REF,NOICF"
"/DEBUG"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libstd.natvis"

c1.49sornl79ds1984x.rcgu.o : error LNK2019: 无法解析的外部符号 add，函数 _ZN2c14main17h3392197dd250b87dE 中引用了该符号
D:\playground\rust\t1\target\debug\examples\c1.exe : fatal error LNK1120: 1 个无法解析的外部命令
*/

/*
"C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.35.32215\\bin\\HostX64\\x64\\link.exe"
"/NOLOGO"
"C:\\Users\\Admin\\AppData\\Local\\Temp\\rustccZ4jhm\\symbols.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.25q9etgh09im62f9.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.2voj6glu3rvsnah2.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.3chm07wctojpe63u.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.49sornl79ds1984x.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.4vua4o01s8fyn7g8.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.e0aij2jsuccc4b4.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.3w3albfpnjxbwew8.rcgu.o"
"/LIBPATH:D:\\playground\\rust\\t1\\target\\debug\\deps"
"/LIBPATH:C:\\Users\\Admin\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_x86_64_msvc-0.52.0\\lib"
"/LIBPATH:C:\\Users\\Admin\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_x86_64_msvc-0.48.5\\lib"
"/LIBPATH:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
"mylib.lib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd-d67e7b8bedb11cdc.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libpanic_unwind-64b1a07f55304d5e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_demangle-f120462e85f67653.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd_detect-d983e2a0a2a0c17a.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libhashbrown-3ef2a86080fe9062.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_alloc-eb4d69e1a344b307.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunwind-5d40568d59362563.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcfg_if-23ecb7d440e7060a.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liblibc-d852afaab40eda7f.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc-20556e3a8d3a4922.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_core-2ad8a2d023c9ae2e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-0731c9d2f2fdad4e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-8ed27bc4ebec64d5.rlib"
"kernel32.lib"
"advapi32.lib"
"bcrypt.lib"
"kernel32.lib"
"ntdll.lib"
"userenv.lib"
"ws2_32.lib"
"kernel32.lib"
"ws2_32.lib"
"kernel32.lib"
"ntdll.lib"
"kernel32.lib"
"msvcrt.lib"
"/NXCOMPAT"
"/LIBPATH:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
"/OUT:D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.exe"
"/OPT:REF,NOICF"
"/DEBUG"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libstd.natvis"

LINK : fatal error LNK1181: 无法打开输入文件“mylib.lib”
*/

/*
"C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.35.32215\\bin\\HostX64\\x64\\link.exe"
"/NOLOGO"
"C:\\Users\\Admin\\AppData\\Local\\Temp\\rustcfZSIbm\\symbols.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.25q9etgh09im62f9.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.2voj6glu3rvsnah2.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.3chm07wctojpe63u.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.49sornl79ds1984x.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.4vua4o01s8fyn7g8.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.e0aij2jsuccc4b4.rcgu.o"
"D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.3w3albfpnjxbwew8.rcgu.o"
"/LIBPATH:D:\\playground\\rust\\t1\\target\\debug\\deps"
"/LIBPATH:C:\\Users\\Admin\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_x86_64_msvc-0.52.0\\lib"
"/LIBPATH:C:\\Users\\Admin\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_x86_64_msvc-0.48.5\\lib"
"/LIBPATH:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
"mylib.lib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd-d67e7b8bedb11cdc.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libpanic_unwind-64b1a07f55304d5e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_demangle-f120462e85f67653.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd_detect-d983e2a0a2a0c17a.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libhashbrown-3ef2a86080fe9062.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_alloc-eb4d69e1a344b307.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunwind-5d40568d59362563.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcfg_if-23ecb7d440e7060a.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liblibc-d852afaab40eda7f.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc-20556e3a8d3a4922.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librustc_std_workspace_core-2ad8a2d023c9ae2e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-0731c9d2f2fdad4e.rlib"
"C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-8ed27bc4ebec64d5.rlib"
"kernel32.lib"
"advapi32.lib"
"bcrypt.lib"
"kernel32.lib"
"ntdll.lib"
"userenv.lib"
"ws2_32.lib"
"kernel32.lib"
"ws2_32.lib"
"kernel32.lib"
"ntdll.lib"
"kernel32.lib"
"msvcrt.lib"
"/NXCOMPAT"
"/LIBPATH:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
"/OUT:D:\\playground\\rust\\t1\\target\\debug\\examples\\c1.exe"
"/OPT:REF,NOICF"
"/DEBUG"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis"
"/NATVIS:C:\\Users\\Admin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libstd.natvis"

c1.49sornl79ds1984x.rcgu.o : error LNK2019: 无法解析的外部符号 __imp_add，函数 _ZN2c14main17h3392197dd250b87dE 中引用了该符号
D:\playground\rust\t1\target\debug\examples\c1.exe : fatal error LNK1120: 1 个无法解析的外部命令
*/