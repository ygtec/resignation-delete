# Resignation-Delete - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: 初始化 Rust 项目和基础结构
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 使用 Cargo 初始化 Rust 项目
  - 配置 Cargo.toml 依赖（使用 egui + eframe 作为 GUI 框架）
  - 创建项目目录结构
- **Acceptance Criteria Addressed**: [FR-2, NFR-3, NFR-4]
- **Test Requirements**:
  - `programmatic` TR-1.1: cargo build 成功编译
  - `programmatic` TR-1.2: cargo run 能启动基础窗口
  - `human-judgement` TR-1.3: 项目结构清晰，有合理的模块划分
- **Notes**: 选择 eframe + egui 作为 GUI 框架，因为它轻量且跨平台

## [ ] Task 2: 实现核心数据模型和扫描器框架
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - 定义 DataItem 数据结构（名称、类型、路径、描述、风险级别等）
  - 定义 Scanner trait 用于统一不同类型的数据扫描
  - 实现扫描协调器
- **Acceptance Criteria Addressed**: [FR-1, AC-1]
- **Test Requirements**:
  - `programmatic` TR-2.1: 可以创建和管理 DataItem
  - `programmatic` TR-2.2: Scanner trait 可以正确扩展
  - `programmatic` TR-2.3: 协调器可以正确调度多个 Scanner
- **Notes**: 为后续各模块的扫描器奠定基础

## [ ] Task 3: 实现 Git SSH 密钥扫描和清除
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 实现 GitSshScanner 扫描 ~/.ssh 目录
  - 识别私钥、公钥、配置文件等
  - 实现对应的清除功能
- **Acceptance Criteria Addressed**: [FR-5, AC-5]
- **Test Requirements**:
  - `programmatic` TR-3.1: 能正确扫描到 SSH 密钥文件
  - `programmatic` TR-3.2: 能安全删除选中的 SSH 文件
  - `human-judgement` TR-3.3: 风险提示清晰明确

## [ ] Task 4: 实现浏览器数据扫描和清除
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 实现 Chrome/Edge 浏览器数据扫描
  - 实现 Firefox 浏览器数据扫描
  - 定位 Cookies、历史记录、书签、登录状态等文件位置
  - 实现清除功能
- **Acceptance Criteria Addressed**: [FR-6, AC-6]
- **Test Requirements**:
  - `programmatic` TR-4.1: 能扫描到常见浏览器用户数据目录
  - `programmatic` TR-4.2: 能识别不同类型的浏览器数据文件
  - `programmatic` TR-4.3: 清除功能能正确删除选中文件
- **Notes**: 需要支持跨平台的浏览器路径

## [ ] Task 5: 实现 JetBrains 系列 IDE 扫描和清除
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 扫描 JetBrains IDE 配置目录
  - 定位登录凭证和用户配置文件
  - 实现清除功能
- **Acceptance Criteria Addressed**: [FR-7, AC-7]
- **Test Requirements**:
  - `programmatic` TR-5.1: 能扫描到 JetBrains IDE 配置目录
  - `programmatic` TR-5.2: 能识别和清除登录凭证
  - `human-judgement` TR-5.3: 风险提示明确

## [ ] Task 6: 实现 VSCode 扫描和清除
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 扫描 VSCode 用户配置和扩展目录
  - 定位登录状态和凭证文件
  - 实现清除功能
- **Acceptance Criteria Addressed**: [FR-10, AC-7]
- **Test Requirements**:
  - `programmatic` TR-6.1: 能扫描到 VSCode 配置目录
  - `programmatic` TR-6.2: 能识别和清除登录凭证

## [ ] Task 7: 实现 AI 工具（Claude、Cursor、Kimi、Qwen 等）扫描和清除
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 扫描常见 AI 工具的配置目录
  - 定位登录凭证和用户数据
  - 实现清除功能
- **Acceptance Criteria Addressed**: [FR-9, AC-8]
- **Test Requirements**:
  - `programmatic` TR-7.1: 能扫描到各 AI 工具的配置目录
  - `programmatic` TR-7.2: 能识别和清除登录凭证

## [ ] Task 8: 实现 WPS 和其他办公软件扫描和清除
- **Priority**: P1
- **Depends On**: Task 2
- **Description**:
  - 扫描 WPS 等办公软件配置目录
  - 定位登录凭证和用户数据
  - 实现清除功能
- **Acceptance Criteria Addressed**: [FR-8]
- **Test Requirements**:
  - `programmatic` TR-8.1: 能扫描到办公软件配置目录
  - `programmatic` TR-8.2: 能识别和清除登录凭证

## [ ] Task 9: 实现完整的 GUI 界面
- **Priority**: P0
- **Depends On**: Task 2, Task 3-8（至少完成部分扫描器）
- **Description**:
  - 使用 eframe + egui 构建主窗口
  - 实现数据项列表展示
  - 实现复选框和全选/取消功能
  - 实现清除按钮和进度显示
  - 实现风险提示和确认对话框
- **Acceptance Criteria Addressed**: [FR-2, FR-3, AC-2, AC-3, NFR-2]
- **Test Requirements**:
  - `human-judgement` TR-9.1: 界面清晰易用
  - `programmatic` TR-9.2: 全选/取消功能正常工作
  - `human-judgement` TR-9.3: 风险提示明显且清晰

## [ ] Task 10: 实现清除引擎和安全验证
- **Priority**: P0
- **Depends On**: Task 2-9
- **Description**:
  - 实现统一的清除引擎
  - 添加确认对话框和警告
  - 实现操作日志记录
  - 确保删除操作安全可靠
- **Acceptance Criteria Addressed**: [FR-4, AC-4]
- **Test Requirements**:
  - `programmatic` TR-10.1: 清除操作能正确删除选中文件
  - `programmatic` TR-10.2: 删除前有确认提示
  - `human-judgement` TR-10.3: 操作过程有进度和状态反馈

## [ ] Task 11: 测试和优化
- **Priority**: P1
- **Depends On**: Task 1-10
- **Description**:
  - 端到端测试所有功能
  - 修复发现的问题
  - 性能优化
  - 文档完善
- **Acceptance Criteria Addressed**: [所有 AC]
- **Test Requirements**:
  - `programmatic` TR-11.1: 所有单元测试通过
  - `human-judgement` TR-11.2: 手动测试所有主要场景
  - `human-judgement` TR-11.3: 代码注释和文档完善
