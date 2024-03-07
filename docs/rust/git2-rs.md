$ cat target/debug/build/libgit2-sys-ab84b34962818d09/output
cargo:rerun-if-env-changed=LIBGIT2_NO_VENDOR
cargo:rerun-if-env-changed=LIBGIT2_NO_PKG_CONFIG
cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG
cargo:rerun-if-env-changed=PKG_CONFIG
cargo:rerun-if-env-changed=LIBGIT2_STATIC
cargo:rerun-if-env-changed=LIBGIT2_DYNAMIC
cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
cargo:rerun-if-env-changed=PKG_CONFIG_PATH
cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
cargo:rerun-if-env-changed=LIBGIT2_STATIC
cargo:rerun-if-env-changed=LIBGIT2_DYNAMIC
cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
cargo:rerun-if-env-changed=PKG_CONFIG_PATH
cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-msvc
cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_msvc
cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
cargo:warning=failed to probe system libgit2: Could not run `PKG_CONFIG_ALLOW_SYSTEM_LIBS=1 PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags libgit2 libgit2 >= 1.7.2 libgit2 < 1.8.0`
The pkg-config command could not be found.

Most likely, you need to install a pkg-config package for your OS.

If you've already installed it, ensure the pkg-config command is one of the
directories in the PATH environment variable.

If you did not expect this build to link to a pre-installed system library,
then check documentation of the libgit2-sys crate for an option to
build the library from source, or disable features or dependencies
that require pkg-config.
cargo:rustc-cfg=libgit2_vendored
libgit2/include\git2\annotated_commit.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\annotated_commit.h
libgit2/include\git2\apply.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\apply.h
libgit2/include\git2\attr.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\attr.h
libgit2/include\git2\blame.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\blame.h
libgit2/include\git2\blob.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\blob.h
libgit2/include\git2\branch.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\branch.h
libgit2/include\git2\buffer.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\buffer.h
libgit2/include\git2\cert.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\cert.h
libgit2/include\git2\checkout.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\checkout.h
libgit2/include\git2\cherrypick.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\cherrypick.h
libgit2/include\git2\clone.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\clone.h
libgit2/include\git2\commit.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\commit.h
libgit2/include\git2\common.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\common.h
libgit2/include\git2\config.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\config.h
libgit2/include\git2\credential.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\credential.h
libgit2/include\git2\credential_helpers.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\credential_helpers.h
libgit2/include\git2\cred_helpers.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\cred_helpers.h
libgit2/include\git2\deprecated.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\deprecated.h
libgit2/include\git2\describe.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\describe.h
libgit2/include\git2\diff.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\diff.h
libgit2/include\git2\email.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\email.h
libgit2/include\git2\errors.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\errors.h
libgit2/include\git2\experimental.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\experimental.h
libgit2/include\git2\filter.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\filter.h
libgit2/include\git2\global.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\global.h
libgit2/include\git2\graph.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\graph.h
libgit2/include\git2\ignore.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\ignore.h
libgit2/include\git2\index.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\index.h
libgit2/include\git2\indexer.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\indexer.h
libgit2/include\git2\mailmap.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\mailmap.h
libgit2/include\git2\merge.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\merge.h
libgit2/include\git2\message.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\message.h
libgit2/include\git2\net.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\net.h
libgit2/include\git2\notes.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\notes.h
libgit2/include\git2\object.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\object.h
libgit2/include\git2\odb.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\odb.h
libgit2/include\git2\odb_backend.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\odb_backend.h
libgit2/include\git2\oid.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\oid.h
libgit2/include\git2\oidarray.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\oidarray.h
libgit2/include\git2\pack.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\pack.h
libgit2/include\git2\patch.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\patch.h
libgit2/include\git2\pathspec.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\pathspec.h
libgit2/include\git2\proxy.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\proxy.h
libgit2/include\git2\rebase.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\rebase.h
libgit2/include\git2\refdb.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\refdb.h
libgit2/include\git2\reflog.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\reflog.h
libgit2/include\git2\refs.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\refs.h
libgit2/include\git2\refspec.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\refspec.h
libgit2/include\git2\remote.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\remote.h
libgit2/include\git2\repository.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\repository.h
libgit2/include\git2\reset.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\reset.h
libgit2/include\git2\revert.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\revert.h
libgit2/include\git2\revparse.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\revparse.h
libgit2/include\git2\revwalk.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\revwalk.h
libgit2/include\git2\signature.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\signature.h
libgit2/include\git2\stash.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\stash.h
libgit2/include\git2\status.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\status.h
libgit2/include\git2\stdint.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\stdint.h
libgit2/include\git2\strarray.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\strarray.h
libgit2/include\git2\submodule.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\submodule.h
libgit2/include\git2\sys\alloc.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\alloc.h
libgit2/include\git2\sys\commit.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\commit.h
libgit2/include\git2\sys\commit_graph.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\commit_graph.h
libgit2/include\git2\sys\config.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\config.h
libgit2/include\git2\sys\cred.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\cred.h
libgit2/include\git2\sys\credential.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\credential.h
libgit2/include\git2\sys\diff.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\diff.h
libgit2/include\git2\sys\email.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\email.h
libgit2/include\git2\sys\filter.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\filter.h
libgit2/include\git2\sys\hashsig.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\hashsig.h
libgit2/include\git2\sys\index.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\index.h
libgit2/include\git2\sys\mempack.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\mempack.h
libgit2/include\git2\sys\merge.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\merge.h
libgit2/include\git2\sys\midx.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\midx.h
libgit2/include\git2\sys\odb_backend.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\odb_backend.h
libgit2/include\git2\sys\openssl.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\openssl.h
libgit2/include\git2\sys\path.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\path.h
libgit2/include\git2\sys\refdb_backend.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\refdb_backend.h
libgit2/include\git2\sys\reflog.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\reflog.h
libgit2/include\git2\sys\refs.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\refs.h
libgit2/include\git2\sys\remote.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\remote.h
libgit2/include\git2\sys\repository.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\repository.h
libgit2/include\git2\sys\stream.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\stream.h
libgit2/include\git2\sys\transport.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\sys\transport.h
libgit2/include\git2\tag.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\tag.h
libgit2/include\git2\trace.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\trace.h
libgit2/include\git2\transaction.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\transaction.h
libgit2/include\git2\transport.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\transport.h
libgit2/include\git2\tree.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\tree.h
libgit2/include\git2\types.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\types.h
libgit2/include\git2\version.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\version.h
libgit2/include\git2\worktree.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2\worktree.h
libgit2/include\git2.h => D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\include\git2.h
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
annotated_commit.c
apply.c
attr_file.c
attr.c
blame.c
attrcache.c
blob.c
blame_git.c
branch.c
buf.c
cache.c
checkout.c
cherrypick.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
clone.c
commit.c
commit_graph.c
commit_list.c
config.c
config_cache.c
config_entries.c
config_file.c
config_parse.c
config_snapshot.c
config_mem.c
crlf.c
delta.c
describe.c
diff.c
diff_driver.c
diff_file.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
diff_generate.c
diff_parse.c
diff_print.c
diff_stats.c
diff_tform.c
diff_xdiff.c
email.c
errors.c
fetch.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
fetchhead.c
filter.c
grafts.c
graph.c
hashsig.c
ident.c
idxmap.c
ignore.c
index.c
indexer.c
iterator.c
libgit2.c
mailmap.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
merge.c
merge_driver.c
merge_file.c
message.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
midx.c
mwindow.c
notes.c
object.c
object_api.c
odb.c
odb_loose.c
odb_mempack.c
odb_pack.c
offmap.c
oid.c
oidarray.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
oidmap.c
pack-objects.c
pack.c
parse.c
patch.c
patch_generate.c
patch_parse.c
path.c
pathspec.c
proxy.c
push.c
reader.c
rebase.c
refdb.c
refdb_fs.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
reflog.c
refs.c
refspec.c
remote.c
repository.c
reset.c
revert.c
revparse.c
revwalk.c
signature.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
stash.c
status.c
strarray.c
submodule.c
sysdir.c
tag.c
threadstate.c
trace.c
trailer.c
transaction.c
transport.c
tree-cache.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
tree.c
worktree.c
alloc.c
date.c
filebuf.c
fs_path.c
futils.c
hash.c
net.c
pool.c
posix.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
pqueue.c
rand.c
runtime.c
regexp.c
sortedcache.c
str.c
strmap.c
thread.c
tsort.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
utf8.c
util.c
varint.c
vector.c
wildmatch.c
zstream.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
auth.c
auth_gssapi.c
auth_ntlmclient.c
auth_sspi.c
credential.c
credential_helpers.c
git.c
http.c
httpclient.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
local.c
smart.c
smart_pkt.c
smart_protocol.c
ssh.c
winhttp.c
mbedtls.c
openssl.c
openssl_dynamic.c
openssl_legacy.c
registry.c
schannel.c
socket.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
stransport.c
tls.c
http_parser.c
xdiffi.c
xemit.c
xhistogram.c
xmerge.c
xpatience.c
xprepare.c
xutils.c
pcre_byte_order.c
pcre_chartables.c
pcre_compile.c
pcre_config.c
pcre_dfa_exec.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
pcre_exec.c
pcre_fullinfo.c
pcre_get.c
pcre_globals.c
pcre_maketables.c
pcre_jit_compile.c
pcre_newline.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
pcre_ord2utf8.c
pcre_printint.c
pcre_refcount.c
pcre_string_utils.c
pcre_study.c
pcre_tables.c
pcre_ucd.c
pcre_valid_utf8.c
pcre_version.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
pcre_xclass.c
pcreposix.c
failalloc.c
stdalloc.c
dir.c
error.c
map.c
path_w32.c
posix_w32.c
precompiled.c
thread.c
utf-conv.c
w32_leakcheck.c
w32_buffer.c
w32_util.c
collisiondetect.c
ubc_check.c
sha1.c
win32.c
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
exit code: 0
cargo:rerun-if-env-changed=AR_x86_64-pc-windows-msvc
AR_x86_64-pc-windows-msvc = None
cargo:rerun-if-env-changed=AR_x86_64_pc_windows_msvc
AR_x86_64_pc_windows_msvc = None
cargo:rerun-if-env-changed=HOST_AR
HOST_AR = None
cargo:rerun-if-env-changed=AR
AR = None
cargo:rerun-if-env-changed=ARFLAGS_x86_64-pc-windows-msvc
ARFLAGS_x86_64-pc-windows-msvc = None
cargo:rerun-if-env-changed=ARFLAGS_x86_64_pc_windows_msvc
ARFLAGS_x86_64_pc_windows_msvc = None
cargo:rerun-if-env-changed=HOST_ARFLAGS
HOST_ARFLAGS = None
cargo:rerun-if-env-changed=ARFLAGS
ARFLAGS = None
cargo:rustc-link-search=native=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.35.32215\atlmfc\lib\x64
cargo:rustc-link-lib=static=git2
cargo:rustc-link-search=native=D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out\build
cargo:root=D:\github\rust-lang\git2-rs\target\debug\build\libgit2-sys-ab84b34962818d09\out
cargo:rustc-link-lib=winhttp
cargo:rustc-link-lib=rpcrt4
cargo:rustc-link-lib=ole32
cargo:rustc-link-lib=crypt32
cargo:rustc-link-lib=secur32
cargo:rerun-if-changed=libgit2/include
cargo:rerun-if-changed=libgit2/src
cargo:rerun-if-changed=libgit2/deps
