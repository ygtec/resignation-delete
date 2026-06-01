# Resignation Delete - 构建指南

## 问题说明

当前系统缺少 MSVC 链接器（`link.exe`），这是编译 Windows 平台 Rust 项目的必需工具。

## 解决方案

### 方案 1：安装 Visual Studio Build Tools（推荐）

1. 下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. 运行安装程序
3. 在"工作负载"中选择 **"使用 C++ 的桌面开发"**
4. 点击安装
5. 安装完成后，重新打开终端并运行：
   ```bash
   cargo build --release
   ```

### 方案 2：安装 Visual Studio Code 扩展

如果您已经安装了 Visual Studio Code，还需要单独安装 Build Tools：
- VS Code 不包含 C++ 构建工具
- 必须安装 Visual Studio Build Tools 或完整的 Visual Studio

### 方案 3：使用 MinGW-w64（替代方案）

如果您想使用 GCC 而不是 MSVC：

1. 安装 MinGW-w64：
   - 下载：https://www.mingw-w64.org/
   - 或使用 Chocolatey：`choco install mingw`

2. 安装 GNU 目标工具链：
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

3. 配置项目使用 GNU 工具链：
   在 `.cargo/config.toml` 中添加：
   ```toml
   [build]
   target = "x86_64-pc-windows-gnu"
   ```

4. 编译项目：
   ```bash
   cargo build --release
   ```

## 验证安装

安装完成后，运行以下命令验证：

```bash
# 检查 Rust 工具链
rustup show

# 检查链接器
where link.exe

# 或检查 GCC
gcc --version

# 编译项目
cargo build --release

# 运行程序
cargo run
```

## 当前状态

项目代码已完全开发完成，包括：
- ✅ 核心数据模型
- ✅ 扫描器框架
- ✅ Git SSH 扫描器
- ✅ 浏览器扫描器
- ✅ JetBrains IDE 扫描器
- ✅ VSCode 扫描器
- ✅ AI 工具扫描器
- ✅ 清除引擎
- ✅ GUI 界面
- ✅ 完整文档

**只需安装构建工具即可编译运行！**

## 快速测试

如果您想快速测试代码是否正确，可以使用 `cargo check`：

```bash
cargo check
```

这会检查代码语法和类型，但不生成可执行文件，速度更快。
