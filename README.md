# Resignation Delete - 离职数据清理工具

一个开源的桌面应用程序，帮助开发人员在离职时安全、快速地清除电脑上的个人信息和账号数据。

[![Build Status](https://github.com/your-username/resignation-delete/actions/workflows/build.yml/badge.svg)](https://github.com/your-username/resignation-delete/actions/workflows/build.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 功能特点

- **一键扫描** - 快速扫描系统中常见的个人数据项
- **可视化选择** - 清晰的 GUI 界面，可选择性地清除数据
- **风险评估** - 对数据项进行风险等级标记（Low/Medium/High/Critical）
- **安全清除** - 支持多次覆写确保数据无法恢复
- **跨平台** - 支持 Windows、macOS 和 Linux

## 支持的数据类型

### 1. Git SSH 密钥
- 扫描 `~/.ssh` 目录
- 识别私钥、公钥和配置文件

### 2. 浏览器数据
- Chrome/Edge：登录状态、Cookie、历史记录、书签
- Firefox：用户配置文件数据

### 3. 开发工具
- **JetBrains 系列**：IntelliJ IDEA、PyCharm、WebStorm 等的配置和许可证
- **VSCode**：用户配置和扩展数据

### 4. AI 工具
- Claude Desktop
- Cursor
- GitHub Copilot
- Kimi
- 通义千问

### 5. 办公软件
- WPS Office
- 其他常见办公软件的配置

## 项目结构

```
resignation-delete/
├── src/
│   ├── main.rs           # 主程序入口，包含 GUI 界面
│   ├── models.rs         # 数据模型定义（DataItem、RiskLevel 等）
│   ├── cleaner.rs        # 数据清除引擎
│   ├── scanner.rs        # 扫描器 trait 定义（保留兼容）
│   ├── coordinator.rs    # 扫描协调器（保留兼容）
│   └── scanners/         # 各类数据扫描器
│       ├── mod.rs        # 扫描器模块导出
│       ├── git_ssh.rs    # Git SSH 扫描器
│       ├── browsers.rs   # 浏览器扫描器
│       ├── jetbrains.rs  # JetBrains IDE 扫描器
│       ├── vscode.rs     # VSCode 扫描器
│       └── ai_tools.rs   # AI 工具扫描器
├── Cargo.toml            # 项目配置和依赖
└── README.md             # 项目说明文档
```

## 安装和运行

### 前置要求

- Rust 1.70+ 工具链
- Windows：需要 Visual Studio Build Tools（包含 C++ 桌面开发选项）

### 构建和运行

```bash
# 克隆或进入项目目录
cd resignation-delete

# 构建项目
cargo build --release

# 运行应用
cargo run
```

### 使用说明

1. **扫描系统** - 点击「扫描系统」按钮开始查找可清理的数据项
2. **选择数据** - 查看扫描结果，使用复选框选择要清除的数据项
   - 默认全选，可取消勾选不想清除的项
3. **确认风险** - 查看高风险警告信息
4. **执行清除** - 点击「清除选中项」，确认后开始执行

## 技术栈

- **语言**：Rust
- **GUI 框架**：eframe + egui
- **依赖**：
  - `dirs` - 跨平台目录路径处理
  - `walkdir` - 目录遍历
  - `rand` - 随机数生成（用于数据覆写）
  - `chrono` - 时间处理
  - `log` + `simple_logger` - 日志系统
  - `thiserror` - 错误处理

## 开发计划

- [ ] 添加更多软件支持
- [ ] 数据备份选项
- [ ] 自定义扫描路径
- [ ] 国际化支持
- [ ] 更多安全选项

## 安全声明

- 本工具仅供学习和个人使用
- 使用前请务必备份重要数据
- 开发者不对数据丢失负责
- 请谨慎操作，确保理解清除的内容

## 许可证

MIT License 或 Apache License 2.0（可选）

## 贡献

欢迎提交 Issue 和 Pull Request！

## 项目规格文档

项目开发规格文档位于 `.trae/specs/` 目录：
- `spec.md` - 产品需求文档
- `tasks.md` - 实施计划和任务分解
- `checklist.md` - 验证清单
