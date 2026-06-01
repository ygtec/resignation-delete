# Resignation-Delete - Product Requirement Document

## Overview
- **Summary**: 开发一个开源桌面应用程序，帮助开发人员在离职时安全、快速地清除电脑上的个人信息和账号数据
- **Purpose**: 解决开发人员离职后忘记清除公司电脑上个人账号信息的问题，保护隐私安全
- **Target Users**: 开发人员、设计人员、使用公司电脑的技术人员

## Goals
- 一键检测和清除常见开发工具的个人数据
- 提供可视化界面，让用户可以选择要清除的内容
- 支持多种常见浏览器、IDE、AI工具和办公软件
- 安全可靠，避免误删重要数据

## Non-Goals (Out of Scope)
- 不支持深度数据恢复擦除（多次覆写）
- 不清除系统级配置或工作相关文件
- 不支持远程操作或自动化清除

## Background & Context
- 开发人员经常使用各种工具和账号，离职时清除不彻底可能导致隐私泄露
- 各种软件的数据存储位置分散，手动清除繁琐且容易遗漏
- Rust语言具有高性能和内存安全特性，适合这类工具开发
- GUI框架选择：考虑使用 egui 或 tauri 实现跨平台桌面应用

## Functional Requirements
- **FR-1**: 扫描和检测系统中的个人数据项
- **FR-2**: 提供GUI界面显示可清除的数据项列表
- **FR-3**: 支持全选/取消全选功能
- **FR-4**: 安全删除选中的数据项
- **FR-5**: 支持清除 Git SSH 密钥
- **FR-6**: 支持清除浏览器（Chrome、Edge、Firefox等）的登录状态、书签和历史记录
- **FR-7**: 支持清除 JetBrains 系列 IDE 的已登录账号
- **FR-8**: 支持清除 WPS 等办公软件的已登录账号
- **FR-9**: 支持清除 AI 工具（Claude、OpenCode、Kimi、Qwen Code、Cursor等）的账号
- **FR-10**: 支持清除 VSCode 的已登录账号

## Non-Functional Requirements
- **NFR-1**: 应用程序应响应迅速，扫描和清除操作不应明显卡顿
- **NFR-2**: 界面清晰易用，操作流程直观
- **NFR-3**: 跨平台支持（Windows、macOS、Linux）
- **NFR-4**: 代码结构清晰，便于维护和扩展

## Constraints
- **Technical**: 使用 Rust 语言开发，GUI 使用 egui 或 tauri 框架
- **Business**: 开源项目，遵循 MIT 或 Apache 2.0 许可证
- **Dependencies**: 需要依赖系统文件操作和可能的注册表访问（Windows）

## Assumptions
- 用户有管理员/root 权限（部分清除操作可能需要）
- 所有目标软件安装在默认路径下
- 清除的数据是可恢复的（应用不会做多次覆写）

## Acceptance Criteria

### AC-1: 系统扫描功能
- **Given**: 用户启动应用程序
- **When**: 应用程序进行系统扫描
- **Then**: 应用程序应列出所有检测到的可清除数据项
- **Verification**: `programmatic`

### AC-2: GUI数据项显示
- **Given**: 扫描完成
- **When**: 用户查看界面
- **Then**: 每个数据项应显示名称、类型、路径和清除风险提示
- **Verification**: `human-judgment`

### AC-3: 全选/取消功能
- **Given**: 数据项列表已显示
- **When**: 用户点击全选或取消全选
- **Then**: 所有数据项的勾选状态应相应切换
- **Verification**: `programmatic`

### AC-4: 安全清除
- **Given**: 用户选择了部分数据项并点击清除
- **When**: 清除操作完成
- **Then**: 选中的数据项应被安全删除，系统状态稳定
- **Verification**: `programmatic`

### AC-5: Git SSH 清除
- **Given**: 用户选择清除 Git SSH 密钥
- **When**: 清除操作执行
- **Then**: ~/.ssh 目录下的私钥和公钥应被检测并可选清除
- **Verification**: `programmatic`

### AC-6: 浏览器数据清除
- **Given**: 浏览器（Chrome/Edge/Firefox）已安装并存在用户数据
- **When**: 用户选择清除浏览器数据
- **Then**: 登录状态、历史记录、书签等应被检测并可选清除
- **Verification**: `programmatic`

### AC-7: IDE 账号清除
- **Given**: JetBrains IDE 或 VSCode 已登录账号
- **When**: 用户选择清除 IDE 账号
- **Then**: 相关配置文件和凭证应被检测并可选清除
- **Verification**: `programmatic`

### AC-8: AI 工具账号清除
- **Given**: AI 工具（Claude、Cursor等）已登录
- **When**: 用户选择清除 AI 工具账号
- **Then**: 相关配置文件和凭证应被检测并可选清除
- **Verification**: `programmatic`

## Open Questions
- [ ] 确定使用哪个 GUI 框架（egui vs tauri）
- [ ] 确定数据备份策略（是否需要在清除前备份）
- [ ] 确认需要支持的软件完整列表
