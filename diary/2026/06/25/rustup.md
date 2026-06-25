`rustup` 是 Rust 官方工具链管理程序，负责：Rust 多版本切换、目标平台安装、组件管理、cargo/rustc/rustfmt/clippy 等工具维护、交叉编译环境配置。

# 一、安装 & 卸载 rustup

## 1. Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装选项：

1. default：默认配置（稳定版、组件齐全）
2. customize：自定义版本、组件、安装路径
3. cancel：退出

安装完成后生效环境变量：

```bash
source "$HOME/.cargo/env"
```

## 2. Windows

官网下载 rustup-init.exe 双击运行。

## 3. 卸载 rustup（彻底删除所有工具链）

```bash
rustup self uninstall
```

## 4. 查看版本

```bash
rustup --version
rustup --help # 全局帮助
```

# 二、核心概念：工具链 toolchain

格式：channel-target

- channel：发行通道 stable / beta / nightly / 固定版本 1.79.0
- target：编译目标平台，默认本机平台如 x86_64-unknown-linux-gnu

示例：

stable-x86_64-unknown-linux-gnu
nightly-aarch64-apple-darwin

# 三、频道管理（channel）

## 1. 安装指定工具链

```bash
# 最新稳定版
rustup toolchain install stable

# 每日构建 nightly
rustup toolchain install nightly

# 指定固定版本
rustup toolchain install 1.78.0
```

## 2. 查看已安装所有工具链

```bash
rustup toolchain list
# 简写
rustup show
```

带 (default) 的是全局默认工具链。

## 3. 设置全局默认工具链

```bash
rustup default nightly
rustup default stable
rustup default 1.78.0
```

## 4. 局部目录覆盖（项目单独使用版本）

进入项目目录执行，生成 `rust-toolchain.toml` 锁定版本：

```bash
rustup override set nightly
# 指定版本
rustup override set 1.79.0
# 取消目录覆盖，使用全局default
rustup override unset
# 查看当前目录覆盖
rustup override list
```

## 5. 删除无用工具链

```bash
rustup toolchain uninstall nightly
rustup toolchain uninstall 1.77.0
```

## 6. 更新所有已安装工具链

```bash
rustup update
# 只更新stable
rustup update stable
# 只更新nightly
rustup update nightly
```

# 四、组件管理（component）

每个工具链自带可选组件，常用：

- `rust-src`：Rust 标准库源码（跳转阅读、离线文档）
- `rustfmt`：代码格式化工具
- `clippy`：静态代码检查、lint 工具
- `rust-analyzer`：IDE 后端（部分环境需要手动装）
- `llvm-tools-preview`：llvm 工具链（objdump、strip 等）

## 1. 查看当前工具链可用组件

```bash
rustup component list
# 只看已安装
rustup component list --installed
```

## 2. 安装组件

```bash
# 给默认工具链安装
rustup component add rustfmt clippy rust-src llvm-tools-preview

# 给指定工具链安装
rustup component add --toolchain nightly rust-src
```

## 3. 卸载组件

```bash
rustup component remove rustfmt
```

## 4. 单独更新组件

跟随 rustup update 自动更新，无需单独命令。

# 五、目标平台（target，交叉编译核心）

## 1. 查看本机默认 target

```bash
rustup show
```

## 2. 列出所有支持的目标

```bash
rustup target list
# 过滤windows
rustup target list | grep windows
```

## 3. 安装交叉编译目标

```bash
# Linux 编译Windows exe
rustup target add x86_64-pc-windows-gnu

# ARM Linux
rustup target add aarch64-unknown-linux-gnu

# WASM
rustup target add wasm32-unknown-unknown
```

## 4. 卸载目标

```bash
rustup target remove x86_64-pc-windows-gnu
```

## 5. 编译时指定 target

```bash
cargo build --target wasm32-unknown-unknown
```

# 六、rustup run：临时切换工具链执行命令

不用修改全局 / 目录默认，单次执行使用指定版本：

```bash
# 用 nightly 执行 cargo build
rustup run nightly cargo build

# 固定版本执行rustc
rustup run 1.78.0 rustc --version

# 运行rustfmt
rustup run nightly rustfmt src/main.rs
```

# 七、文档 & 源码相关命令

## 1. 打开本地官方文档（离线）

```bash
rustup doc
# 打开标准库文档
rustup doc --std
# 打开指定crate文档（需先cargo doc）
rustup doc --crate sqlx
```

前提：安装 `rust-src` 组件才能跳转源码。

# 八、环境 & 路径管理

## 1. 查看 rustup 环境信息

```bash
rustup show
# 输出：默认工具链、已装target、组件、安装目录、环境变量
```

## 2. 打印环境变量（给 shell 加载）

```bash
rustup env
# 直接输出PATH配置
rustup env PATH
```

## 3. 自定义安装目录（安装时配置）

```bash
默认 ~/.cargo，安装时可指定 RUSTUP_HOME。
```

# 九、工具链别名（方便简写）

```bash
# 创建别名 my-nightly 指向 nightly
rustup toolchain link my-nightly ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
# 使用别名
rustup run my-nightly cargo check
```

# 十、常用实用组合示例

## 1. 全新环境一键配置开发环境

```bash
# 安装稳定版 + 全套组件
rustup toolchain install stable
rustup component add rustfmt clippy rust-src llvm-tools-preview
# 安装WASM目标
rustup target add wasm32-unknown-unknown
```

## 2. 项目固定 nightly 版本

```bash
cd my-project
rustup override set nightly
# 后续所有cargo命令自动使用nightly
cargo build
cargo clippy
```

## 3. 本地多版本测试

```bash
rustup run 1.76.0 cargo test
rustup run stable cargo test
rustup run nightly cargo test
```

## 4. 更新并清理旧版本

```bash
rustup update
rustup toolchain uninstall 1.75.0
```

# 十一、常见问题命令

1. 修复工具链损坏

```bash
rustup self repair
```

2. 清理缓存安装包

```bash
rustup cache clean
# 指定清理旧版本缓存
rustup cache clean --max-age 30d
```

3. 查看 rustup 自身更新

```bash
rustup self update
```

# 十二、完整命令速查表

```bash
# 安装/卸载
rustup self install
rustup self uninstall
rustup self update
rustup self repair

# 工具链管理
rustup toolchain install <chain>
rustup toolchain list
rustup toolchain uninstall <chain>
rustup default <chain>
rustup override set/unset/list
rustup update

# 组件
rustup component list
rustup component add xxx
rustup component remove xxx

# 交叉编译target
rustup target list
rustup target add xxx
rustup target remove xxx

# 运行指定工具链
rustup run <chain> <cmd>

# 文档
rustup doc

# 信息查看
rustup show
rustup env

# 缓存清理
rustup cache clean
```
