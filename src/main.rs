use eframe::{egui, App, CreationContext, Frame, NativeOptions, run_native};
use egui::{Color32, Context, FontData, FontDefinitions, FontFamily, RichText, Rounding, Stroke, Style, Vec2, Visuals, Window, Button, ScrollArea, Separator, Ui, Layout, Align, Margin};
use std::time::SystemTime;
use chrono::Local;

mod models;
mod cleaner;
mod scanners;

use models::{DataItem, DataType, RiskLevel};
use cleaner::{Cleaner, CleanLog, LogLevel, CleanTask, CleanStatus};
use scanners::{Scanner, git_ssh::GitSshScanner, browsers::BrowsersScanner, jetbrains::JetBrainsScanner, vscode::VSCodeScanner, ai_tools::AIToolsScanner};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Cleanup,
    Logs,
    Settings,
}

#[derive(Debug, Clone)]
pub struct CategoryItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub size: u64,
    pub risk_level: RiskLevel,
    pub selected: bool,
    pub scanned: bool,
    pub detected: bool,
}

impl CategoryItem {
    pub fn new(id: &str, name: &str, description: &str, path: &str, size: u64, risk_level: RiskLevel) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            path: path.to_string(),
            size,
            risk_level,
            selected: false,
            scanned: false,
            detected: false,
        }
    }
}

pub struct Category {
    pub name: String,
    pub emoji: String,
    pub color: Color32,
    pub items: Vec<CategoryItem>,
    pub expanded: bool,
}

impl Category {
    pub fn new(name: &str, emoji: &str, color: Color32, items: Vec<CategoryItem>) -> Self {
        Self {
            name: name.to_string(),
            emoji: emoji.to_string(),
            color,
            items,
            expanded: true,
        }
    }
}

pub struct ResignationApp {
    pub current_tab: Tab,
    pub categories: Vec<Category>,
    pub selected_count: usize,
    pub total_size: u64,
    pub show_confirm_dialog: bool,
    pub show_result_dialog: bool,
    pub cleanup_result: Option<String>,
    pub logs: Vec<CleanLog>,
    pub cleaner: Cleaner,
    pub scan_in_progress: bool,
    pub scan_progress: f32,
    pub overwrite_passes: u8,
    pub dark_mode: bool,
}

impl ResignationApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let mut app = Self {
            current_tab: Tab::Cleanup,
            categories: Self::initialize_categories(),
            selected_count: 0,
            total_size: 0,
            show_confirm_dialog: false,
            show_result_dialog: false,
            cleanup_result: None,
            logs: Vec::new(),
            cleaner: Cleaner::new(),
            scan_in_progress: false,
            scan_progress: 0.0,
            overwrite_passes: 3,
            dark_mode: true,
        };
        app.update_selected_count();
        app
    }

    fn initialize_categories() -> Vec<Category> {
        let dev_color = Color32::from_rgb(59, 130, 246);
        let browser_color = Color32::from_rgb(34, 197, 94);
        let ai_color = Color32::from_rgb(168, 85, 247);
        let office_color = Color32::from_rgb(249, 115, 22);

        vec![
            Category::new("开发工具", "💻", dev_color, Self::dev_tools_items()),
            Category::new("浏览器", "🌐", browser_color, Self::browser_items()),
            Category::new("AI工具", "🤖", ai_color, Self::ai_tools_items()),
            Category::new("办公软件", "📊", office_color, Self::office_items()),
        ]
    }

    fn dev_tools_items() -> Vec<CategoryItem> {
        vec![
            CategoryItem::new("dev1", "VSCode", "Visual Studio Code 编辑器配置", "~/.vscode", 52428800, RiskLevel::Medium),
            CategoryItem::new("dev2", "IntelliJ IDEA", "JetBrains IDE 配置与缓存", "~/.IntelliJIdea*", 104857600, RiskLevel::Medium),
            CategoryItem::new("dev3", "PyCharm", "Python IDE 配置与缓存", "~/.PyCharm*", 78643200, RiskLevel::Medium),
            CategoryItem::new("dev4", "WebStorm", "Web IDE 配置与缓存", "~/.WebStorm*", 62914560, RiskLevel::Medium),
            CategoryItem::new("dev5", "VSCode Extensions", "VSCode 扩展插件", "~/.vscode/extensions", 157286400, RiskLevel::Low),
            CategoryItem::new("dev6", "Node Modules", "Node.js 依赖包缓存", "~/node_modules", 209715200, RiskLevel::Low),
            CategoryItem::new("dev7", "npm Cache", "npm 包管理器缓存", "~/.npm", 52428800, RiskLevel::Low),
            CategoryItem::new("dev8", "yarn Cache", "yarn 包管理器缓存", "~/.yarn", 41943040, RiskLevel::Low),
            CategoryItem::new("dev9", "pnpm Cache", "pnpm 包管理器缓存", "~/.pnpm-store", 62914560, RiskLevel::Low),
            CategoryItem::new("dev10", "Cargo Cache", "Rust 包管理器缓存", "~/.cargo", 314572800, RiskLevel::Low),
            CategoryItem::new("dev11", "Rustup", "Rust 工具链", "~/.rustup", 524288000, RiskLevel::Low),
            CategoryItem::new("dev12", "Go Modules", "Go 依赖缓存", "~/go/pkg/mod", 157286400, RiskLevel::Low),
            CategoryItem::new("dev13", "Gradle Cache", "Gradle 构建缓存", "~/.gradle", 104857600, RiskLevel::Low),
            CategoryItem::new("dev14", "Maven Cache", "Maven 依赖缓存", "~/.m2", 78643200, RiskLevel::Low),
            CategoryItem::new("dev15", "Docker", "Docker 容器与镜像数据", "~/.docker", 524288000, RiskLevel::High),
            CategoryItem::new("dev16", "Git Config", "Git 全局配置文件", "~/.gitconfig", 4096, RiskLevel::Critical),
            CategoryItem::new("dev17", "SSH Keys", "SSH 私钥与配置", "~/.ssh", 16384, RiskLevel::Critical),
            CategoryItem::new("dev18", "GNUPG", "GPG 密钥与配置", "~/.gnupg", 32768, RiskLevel::Critical),
            CategoryItem::new("dev19", "AWS CLI", "AWS 命令行配置", "~/.aws", 8192, RiskLevel::Critical),
            CategoryItem::new("dev20", "Azure CLI", "Azure 命令行配置", "~/.azure", 8192, RiskLevel::Critical),
            CategoryItem::new("dev21", "gcloud", "Google Cloud 配置", "~/.config/gcloud", 16384, RiskLevel::Critical),
            CategoryItem::new("dev22", "kubectl", "Kubernetes 配置", "~/.kube", 8192, RiskLevel::Critical),
            CategoryItem::new("dev23", "Terraform", "Terraform 状态与配置", "~/.terraform.d", 32768, RiskLevel::High),
            CategoryItem::new("dev24", "Vagrant", "Vagrant 虚拟机配置", "~/.vagrant.d", 104857600, RiskLevel::Medium),
            CategoryItem::new("dev25", "VirtualBox", "VirtualBox 虚拟机", "~/VirtualBox VMs", 1073741824, RiskLevel::Medium),
            CategoryItem::new("dev26", "VMware", "VMware 虚拟机", "~/Documents/Virtual Machines", 2147483648, RiskLevel::Medium),
            CategoryItem::new("dev27", "Parallels", "Parallels 虚拟机", "~/Parallels", 1073741824, RiskLevel::Medium),
            CategoryItem::new("dev28", "Xcode", "Xcode 开发者工具数据", "~/Library/Developer/Xcode", 524288000, RiskLevel::Medium),
            CategoryItem::new("dev29", "Android SDK", "Android 开发工具包", "~/Android/Sdk", 1073741824, RiskLevel::Medium),
            CategoryItem::new("dev30", "Flutter", "Flutter SDK 与缓存", "~/flutter", 314572800, RiskLevel::Low),
            CategoryItem::new("dev31", "Dart", "Dart 包缓存", "~/.pub-cache", 52428800, RiskLevel::Low),
            CategoryItem::new("dev32", "Unity", "Unity 编辑器配置", "~/Library/Unity", 262144000, RiskLevel::Medium),
        ]
    }

    fn browser_items() -> Vec<CategoryItem> {
        vec![
            CategoryItem::new("browser1", "Chrome", "Google Chrome 浏览器数据", "~/AppData/Local/Google/Chrome", 262144000, RiskLevel::High),
            CategoryItem::new("browser2", "Chrome History", "Chrome 浏览历史", "~/AppData/Local/Google/Chrome/User Data/Default/History", 10485760, RiskLevel::Critical),
            CategoryItem::new("browser3", "Chrome Cookies", "Chrome Cookie 数据", "~/AppData/Local/Google/Chrome/User Data/Default/Cookies", 5242880, RiskLevel::Critical),
            CategoryItem::new("browser4", "Chrome Passwords", "Chrome 保存的密码", "~/AppData/Local/Google/Chrome/User Data/Default/Login Data", 2097152, RiskLevel::Critical),
            CategoryItem::new("browser5", "Edge", "Microsoft Edge 浏览器数据", "~/AppData/Local/Microsoft/Edge", 209715200, RiskLevel::High),
            CategoryItem::new("browser6", "Edge History", "Edge 浏览历史", "~/AppData/Local/Microsoft/Edge/User Data/Default/History", 10485760, RiskLevel::Critical),
            CategoryItem::new("browser7", "Edge Cookies", "Edge Cookie 数据", "~/AppData/Local/Microsoft/Edge/User Data/Default/Cookies", 5242880, RiskLevel::Critical),
            CategoryItem::new("browser8", "Edge Passwords", "Edge 保存的密码", "~/AppData/Local/Microsoft/Edge/User Data/Default/Login Data", 2097152, RiskLevel::Critical),
            CategoryItem::new("browser9", "Firefox", "Mozilla Firefox 浏览器数据", "~/AppData/Roaming/Mozilla/Firefox", 157286400, RiskLevel::High),
            CategoryItem::new("browser10", "Firefox History", "Firefox 浏览历史", "~/AppData/Roaming/Mozilla/Firefox/Profiles/*/places.sqlite", 10485760, RiskLevel::Critical),
            CategoryItem::new("browser11", "Firefox Cookies", "Firefox Cookie 数据", "~/AppData/Roaming/Mozilla/Firefox/Profiles/*/cookies.sqlite", 5242880, RiskLevel::Critical),
            CategoryItem::new("browser12", "Firefox Passwords", "Firefox 保存的密码", "~/AppData/Roaming/Mozilla/Firefox/Profiles/*/logins.json", 2097152, RiskLevel::Critical),
            CategoryItem::new("browser13", "Safari", "Apple Safari 浏览器数据", "~/Library/Safari", 104857600, RiskLevel::High),
            CategoryItem::new("browser14", "Safari History", "Safari 浏览历史", "~/Library/Safari/History.db", 10485760, RiskLevel::Critical),
            CategoryItem::new("browser15", "Opera", "Opera 浏览器数据", "~/AppData/Roaming/Opera Software", 157286400, RiskLevel::High),
            CategoryItem::new("browser16", "Opera GX", "Opera GX 浏览器数据", "~/AppData/Roaming/Opera Software/Opera GX Stable", 157286400, RiskLevel::High),
            CategoryItem::new("browser17", "Brave", "Brave 浏览器数据", "~/AppData/Local/BraveSoftware/Brave-Browser", 209715200, RiskLevel::High),
            CategoryItem::new("browser18", "Vivaldi", "Vivaldi 浏览器数据", "~/AppData/Local/Vivaldi", 157286400, RiskLevel::High),
            CategoryItem::new("browser19", "Tor Browser", "Tor 浏览器数据", "~/Desktop/Tor Browser", 104857600, RiskLevel::Critical),
            CategoryItem::new("browser20", "Chrome Dev", "Chrome 开发者版本数据", "~/AppData/Local/Google/Chrome Dev", 262144000, RiskLevel::High),
            CategoryItem::new("browser21", "Chrome Beta", "Chrome 测试版本数据", "~/AppData/Local/Google/Chrome Beta", 262144000, RiskLevel::High),
            CategoryItem::new("browser22", "Chrome Canary", "Chrome 金丝雀版本数据", "~/AppData/Local/Google/Chrome SxS", 262144000, RiskLevel::High),
            CategoryItem::new("browser23", "Firefox Dev", "Firefox 开发者版本", "~/AppData/Roaming/Mozilla/Firefox/Dev", 157286400, RiskLevel::High),
            CategoryItem::new("browser24", "Firefox Nightly", "Firefox 每夜构建版本", "~/AppData/Roaming/Mozilla/Firefox/Nightly", 157286400, RiskLevel::High),
            CategoryItem::new("browser25", "Waterfox", "Waterfox 浏览器数据", "~/AppData/Roaming/Waterfox", 157286400, RiskLevel::High),
            CategoryItem::new("browser26", "Pale Moon", "Pale Moon 浏览器数据", "~/AppData/Roaming/Moonchild Productions", 157286400, RiskLevel::High),
            CategoryItem::new("browser27", "SeaMonkey", "SeaMonkey 浏览器数据", "~/AppData/Roaming/Mozilla/SeaMonkey", 157286400, RiskLevel::High),
            CategoryItem::new("browser28", "Maxthon", "傲游浏览器数据", "~/AppData/Roaming/Maxthon", 104857600, RiskLevel::High),
            CategoryItem::new("browser29", "360 Safe", "360安全浏览器数据", "~/AppData/Roaming/360se6", 104857600, RiskLevel::High),
            CategoryItem::new("browser30", "QQ Browser", "QQ浏览器数据", "~/AppData/Roaming/Tencent/QQBrowser", 104857600, RiskLevel::High),
            CategoryItem::new("browser31", "UC Browser", "UC浏览器数据", "~/AppData/Local/UCBrowser", 104857600, RiskLevel::High),
            CategoryItem::new("browser32", "Sogou", "搜狗浏览器数据", "~/AppData/Roaming/SogouExplorer", 104857600, RiskLevel::High),
        ]
    }

    fn ai_tools_items() -> Vec<CategoryItem> {
        vec![
            CategoryItem::new("ai1", "Cursor", "Cursor AI 编辑器配置", "~/.cursor", 52428800, RiskLevel::High),
            CategoryItem::new("ai2", "Cursor Cache", "Cursor AI 缓存数据", "~/AppData/Roaming/Cursor", 104857600, RiskLevel::High),
            CategoryItem::new("ai3", "Claude", "Anthropic Claude 配置", "~/.claude", 26214400, RiskLevel::High),
            CategoryItem::new("ai4", "Claude Desktop", "Claude 桌面应用数据", "~/AppData/Roaming/Claude", 52428800, RiskLevel::High),
            CategoryItem::new("ai5", "ChatGPT", "OpenAI ChatGPT 桌面应用", "~/AppData/Roaming/ChatGPT", 52428800, RiskLevel::High),
            CategoryItem::new("ai6", "GitHub Copilot", "GitHub Copilot AI 助手", "~/.github-copilot", 26214400, RiskLevel::High),
            CategoryItem::new("ai7", "Copilot VSCode", "VSCode Copilot 扩展", "~/.vscode/extensions/github.copilot*", 104857600, RiskLevel::High),
            CategoryItem::new("ai8", "Tabnine", "Tabnine AI 代码补全", "~/.tabnine", 52428800, RiskLevel::Medium),
            CategoryItem::new("ai9", "Codeium", "Codeium AI 代码助手", "~/.codeium", 52428800, RiskLevel::Medium),
            CategoryItem::new("ai10", "Kite", "Kite AI 代码补全", "~/.kite", 104857600, RiskLevel::Medium),
            CategoryItem::new("ai11", "Kimi", "Kimi AI 助手配置", "~/.kimi", 26214400, RiskLevel::High),
            CategoryItem::new("ai12", "Kimi Desktop", "Kimi 桌面应用数据", "~/AppData/Roaming/Kimi", 52428800, RiskLevel::High),
            CategoryItem::new("ai13", "Qwen", "通义千问 AI 配置", "~/.qwen", 26214400, RiskLevel::High),
            CategoryItem::new("ai14", "Tongyi", "通义 AI 助手数据", "~/AppData/Roaming/Tongyi", 52428800, RiskLevel::High),
            CategoryItem::new("ai15", "Wenxin", "文心一言配置", "~/.wenxin", 26214400, RiskLevel::High),
            CategoryItem::new("ai16", "Doubao", "豆包 AI 助手", "~/.doubao", 26214400, RiskLevel::High),
            CategoryItem::new("ai17", "Zhipu", "智谱 AI 配置", "~/.zhipu", 26214400, RiskLevel::High),
            CategoryItem::new("ai18", "Baichuan", "百川智能配置", "~/.baichuan", 26214400, RiskLevel::High),
            CategoryItem::new("ai19", "DeepSeek", "DeepSeek AI 配置", "~/.deepseek", 26214400, RiskLevel::High),
            CategoryItem::new("ai20", "OpenCode", "OpenCode AI 编辑器", "~/.opencode", 52428800, RiskLevel::High),
            CategoryItem::new("ai21", "Trae", "Trae AI 编辑器", "~/.trae", 52428800, RiskLevel::High),
            CategoryItem::new("ai22", "Windsurf", "Windsurf AI 编辑器", "~/.windsurf", 52428800, RiskLevel::High),
            CategoryItem::new("ai23", "Aider", "Aider AI 编程助手", "~/.aider", 26214400, RiskLevel::Medium),
            CategoryItem::new("ai24", "Continue", "Continue AI 扩展", "~/.continue", 52428800, RiskLevel::Medium),
            CategoryItem::new("ai25", "Supermaven", "Supermaven AI 助手", "~/.supermaven", 26214400, RiskLevel::Medium),
            CategoryItem::new("ai26", "Cody", "Sourcegraph Cody AI", "~/.sourcegraph", 52428800, RiskLevel::Medium),
            CategoryItem::new("ai27", "JetBrains AI", "JetBrains AI 助手", "~/.jetbrains-ai", 52428800, RiskLevel::Medium),
            CategoryItem::new("ai28", "Amazon CodeWhisperer", "AWS CodeWhisperer", "~/.aws/codewhisperer", 26214400, RiskLevel::High),
            CategoryItem::new("ai29", "Google Gemini", "Gemini AI 配置", "~/.gemini", 26214400, RiskLevel::High),
            CategoryItem::new("ai30", "Perplexity", "Perplexity AI 应用", "~/AppData/Roaming/Perplexity", 52428800, RiskLevel::High),
            CategoryItem::new("ai31", "Hugging Face", "Hugging Face 缓存", "~/.cache/huggingface", 524288000, RiskLevel::Medium),
            CategoryItem::new("ai32", "Ollama", "Ollama 本地模型", "~/.ollama", 1073741824, RiskLevel::Medium),
        ]
    }

    fn office_items() -> Vec<CategoryItem> {
        vec![
            CategoryItem::new("office1", "Microsoft Word", "Word 文档与模板", "~/Documents/Word", 104857600, RiskLevel::Medium),
            CategoryItem::new("office2", "Microsoft Excel", "Excel 表格与模板", "~/Documents/Excel", 104857600, RiskLevel::Medium),
            CategoryItem::new("office3", "Microsoft PowerPoint", "PPT 演示文稿", "~/Documents/PowerPoint", 209715200, RiskLevel::Medium),
            CategoryItem::new("office4", "Microsoft Outlook", "Outlook 邮件数据", "~/AppData/Local/Microsoft/Outlook", 262144000, RiskLevel::Critical),
            CategoryItem::new("office5", "Outlook PST", "Outlook 数据文件", "~/Documents/Outlook Files", 524288000, RiskLevel::Critical),
            CategoryItem::new("office6", "Microsoft Teams", "Teams 缓存与日志", "~/AppData/Roaming/Microsoft/Teams", 157286400, RiskLevel::High),
            CategoryItem::new("office7", "OneNote", "OneNote 笔记数据", "~/AppData/Local/Microsoft/OneNote", 104857600, RiskLevel::Medium),
            CategoryItem::new("office8", "OneDrive", "OneDrive 同步缓存", "~/OneDrive", 1073741824, RiskLevel::High),
            CategoryItem::new("office9", "SharePoint", "SharePoint 同步数据", "~/SharePoint", 524288000, RiskLevel::High),
            CategoryItem::new("office10", "WPS Office", "WPS 办公软件数据", "~/AppData/Local/Kingsoft", 157286400, RiskLevel::Medium),
            CategoryItem::new("office11", "WPS Cloud", "WPS 云文档缓存", "~/AppData/Local/Kingsoft/wpscloud", 104857600, RiskLevel::High),
            CategoryItem::new("office12", "LibreOffice", "LibreOffice 配置", "~/.config/libreoffice", 52428800, RiskLevel::Medium),
            CategoryItem::new("office13", "OpenOffice", "OpenOffice 配置", "~/.openoffice", 52428800, RiskLevel::Medium),
            CategoryItem::new("office14", "Google Drive", "Google Drive 本地同步", "~/Google Drive", 1073741824, RiskLevel::High),
            CategoryItem::new("office15", "Dropbox", "Dropbox 同步文件夹", "~/Dropbox", 1073741824, RiskLevel::High),
            CategoryItem::new("office16", "Box Drive", "Box 云盘同步", "~/Box", 524288000, RiskLevel::High),
            CategoryItem::new("office17", "iCloud Drive", "iCloud 云盘数据", "~/Library/Mobile Documents", 1073741824, RiskLevel::High),
            CategoryItem::new("office18", "Notion", "Notion 本地缓存", "~/AppData/Roaming/Notion", 104857600, RiskLevel::Medium),
            CategoryItem::new("office19", "Obsidian", "Obsidian 笔记库", "~/Obsidian", 262144000, RiskLevel::Medium),
            CategoryItem::new("office20", "Evernote", "印象笔记本地数据", "~/AppData/Roaming/Evernote", 157286400, RiskLevel::Medium),
            CategoryItem::new("office21", "Slack", "Slack 工作区数据", "~/AppData/Roaming/Slack", 262144000, RiskLevel::High),
            CategoryItem::new("office22", "Discord", "Discord 应用数据", "~/AppData/Roaming/Discord", 157286400, RiskLevel::High),
            CategoryItem::new("office23", "Zoom", "Zoom 会议数据", "~/AppData/Roaming/Zoom", 104857600, RiskLevel::Medium),
            CategoryItem::new("office24", "Tencent Meeting", "腾讯会议数据", "~/AppData/Roaming/Tencent/WeMeet", 104857600, RiskLevel::Medium),
            CategoryItem::new("office25", "DingTalk", "钉钉工作数据", "~/AppData/Roaming/DingTalk", 157286400, RiskLevel::High),
            CategoryItem::new("office26", "WeChat Work", "企业微信数据", "~/AppData/Roaming/Tencent/WXWork", 262144000, RiskLevel::Critical),
            CategoryItem::new("office27", "Feishu", "飞书工作数据", "~/AppData/Roaming/Feishu", 157286400, RiskLevel::High),
            CategoryItem::new("office28", "Lark", "Lark 工作数据", "~/AppData/Roaming/Lark", 157286400, RiskLevel::High),
            CategoryItem::new("office29", "Adobe Acrobat", "Adobe PDF 阅读器数据", "~/AppData/Roaming/Adobe/Acrobat", 104857600, RiskLevel::Medium),
            CategoryItem::new("office30", "Foxit Reader", "福昕阅读器数据", "~/AppData/Roaming/Foxit Software", 52428800, RiskLevel::Medium),
            CategoryItem::new("office31", "Thunderbird", "Thunderbird 邮件", "~/AppData/Roaming/Thunderbird", 262144000, RiskLevel::Critical),
            CategoryItem::new("office32", "Apple Mail", "Apple 邮件数据", "~/Library/Mail", 524288000, RiskLevel::Critical),
        ]
    }

    pub fn select_all(&mut self, category_index: usize, select: bool) {
        if let Some(category) = self.categories.get_mut(category_index) {
            for item in &mut category.items {
                item.selected = select;
            }
        }
        self.update_selected_count();
    }

    pub fn update_selected_count(&mut self) {
        self.selected_count = 0;
        self.total_size = 0;
        for category in &self.categories {
            for item in &category.items {
                if item.selected {
                    self.selected_count += 1;
                    self.total_size += item.size;
                }
            }
        }
    }

    pub fn perform_scan(&mut self) {
        self.scan_in_progress = true;
        self.scan_progress = 0.0;

        let scanners: Vec<Box<dyn Scanner>> = vec![
            Box::new(GitSshScanner),
            Box::new(BrowsersScanner),
            Box::new(JetBrainsScanner),
            Box::new(VSCodeScanner),
            Box::new(AIToolsScanner),
        ];

        let total_scanners = scanners.len();
        for (i, scanner) in scanners.iter().enumerate() {
            let items = scanner.scan();
            self.scan_progress = ((i + 1) as f32 / total_scanners as f32) * 100.0;

            for scanned_item in items {
                self.update_item_from_scan(&scanned_item);
            }
        }

        self.scan_in_progress = false;
        self.scan_progress = 100.0;
    }

    fn update_item_from_scan(&mut self, scanned_item: &DataItem) {
        for category in &mut self.categories {
            for item in &mut category.items {
                if item.path == scanned_item.path || item.name.to_lowercase().contains(&scanned_item.path.to_lowercase()) {
                    item.scanned = true;
                    item.detected = true;
                    item.size = scanned_item.size;
                }
            }
        }
    }

    pub fn execute_cleanup(&mut self) {
        let mut items_to_clean: Vec<DataItem> = Vec::new();

        for category in &self.categories {
            for item in &category.items {
                if item.selected {
                    let data_item = DataItem::new(
                        item.id.clone(),
                        item.path.clone(),
                        DataType::Other,
                        item.risk_level,
                        item.size,
                        SystemTime::now(),
                        SystemTime::now(),
                    );
                    items_to_clean.push(data_item);
                }
            }
        }

        if items_to_clean.is_empty() {
            self.cleanup_result = Some("没有选择任何项目".to_string());
            self.show_result_dialog = true;
            return;
        }

        self.cleaner.clear_tasks();
        self.cleaner.add_tasks(items_to_clean);

        let result = self.cleaner.clean_all(|_tasks| true);

        match result {
            Ok(_) => {
                let success_count = self.cleaner.tasks().iter().filter(|t| matches!(t.status, CleanStatus::Completed)).count();
                let fail_count = self.cleaner.tasks().iter().filter(|t| matches!(t.status, CleanStatus::Failed(_))).count();
                self.cleanup_result = Some(format!("清理完成: {} 成功, {} 失败", success_count, fail_count));
            }
            Err(e) => {
                self.cleanup_result = Some(format!("清理出错: {}", e));
            }
        }

        self.logs = self.cleaner.logs().to_vec();
        self.show_result_dialog = true;
    }

    pub fn format_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        if size == 0 {
            return "0 B".to_string();
        }
        let exp = (size as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
        let value = size as f64 / 1024f64.powi(exp as i32);
        if exp == 0 {
            format!("{} {}", size, UNITS[exp])
        } else {
            format!("{:.2} {}", value, UNITS[exp])
        }
    }

    pub fn draw_categories(&mut self, ui: &mut Ui) {
        for cat_idx in 0..self.categories.len() {
            let category = &self.categories[cat_idx];
            let cat_color = category.color;
            let cat_name = category.name.clone();
            let cat_emoji = category.emoji.clone();
            let is_expanded = category.expanded;
            let item_count = category.items.len();
            let selected_in_cat = category.items.iter().filter(|i| i.selected).count();

            ui.horizontal(|ui| {
                let header_text = if is_expanded {
                    format!("▼ {} {} ({} / {})", cat_emoji, cat_name, selected_in_cat, item_count)
                } else {
                    format!("▶ {} {} ({} / {})", cat_emoji, cat_name, selected_in_cat, item_count)
                };

                let header_button = Button::new(RichText::new(header_text).color(cat_color).strong().size(16.0))
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE);

                if ui.add(header_button).clicked() {
                    if let Some(cat) = self.categories.get_mut(cat_idx) {
                        cat.expanded = !cat.expanded;
                    }
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.small_button("全不选").clicked() {
                        self.select_all(cat_idx, false);
                    }
                    if ui.small_button("全选").clicked() {
                        self.select_all(cat_idx, true);
                    }
                });
            });

            if is_expanded {
                ui.indent("cat_".to_string() + &cat_idx.to_string(), |ui| {
                    for item_idx in 0..self.categories[cat_idx].items.len() {
                        let item = &self.categories[cat_idx].items[item_idx];
                        let item_id = item.id.clone();
                        let item_name = item.name.clone();
                        let item_desc = item.description.clone();
                        let item_size = item.size;
                        let item_selected = item.selected;
                        let item_scanned = item.scanned;
                        let item_detected = item.detected;
                        let item_risk = item.risk_level;

                        ui.horizontal(|ui| {
                            let mut checked = item_selected;
                            if ui.checkbox(&mut checked, "").changed() {
                                if let Some(cat) = self.categories.get_mut(cat_idx) {
                                    if let Some(it) = cat.items.get_mut(item_idx) {
                                        it.selected = checked;
                                    }
                                }
                                self.update_selected_count();
                            }

                            let risk_color = match item_risk {
                                RiskLevel::Critical => Color32::from_rgb(239, 68, 68),
                                RiskLevel::High => Color32::from_rgb(249, 115, 22),
                                RiskLevel::Medium => Color32::from_rgb(234, 179, 8),
                                RiskLevel::Low => Color32::from_rgb(34, 197, 94),
                            };

                            let status_indicator = if item_detected {
                                "●"
                            } else if item_scanned {
                                "○"
                            } else {
                                " "
                            };

                            ui.colored_label(risk_color, status_indicator);

                            ui.label(RichText::new(&item_name).strong().size(14.0));
                            ui.label(RichText::new(&item_desc).color(Color32::GRAY).size(12.0));

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new(Self::format_size(item_size)).monospace().size(12.0));
                            });
                        });

                        ui.separator();
                    }
                });
            }

            ui.add_space(8.0);
        }
    }

    pub fn draw_confirm_dialog(&mut self, ctx: &Context) {
        if self.show_confirm_dialog {
            let mut show = self.show_confirm_dialog;
            Window::new("确认清理")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(format!("您选择了 {} 个项目，总计 {}", self.selected_count, Self::format_size(self.total_size)));
                    ui.label("确定要清理这些项目吗？此操作不可撤销！");
                    ui.label("");
                    ui.horizontal(|ui| {
                        if ui.button("  取消  ").clicked() {
                            show = false;
                        }
                        if ui.button("  确认清理  ").clicked() {
                            self.execute_cleanup();
                            show = false;
                        }
                    });
                });
            self.show_confirm_dialog = show;
        }
    }

    pub fn draw_result_dialog(&mut self, ctx: &Context) {
        if self.show_result_dialog {
            let mut show = self.show_result_dialog;
            Window::new("清理结果")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    if let Some(ref result) = self.cleanup_result {
                        ui.label(result);
                    }
                    ui.horizontal(|ui| {
                        if ui.button("  确定  ").clicked() {
                            show = false;
                        }
                    });
                });
            self.show_result_dialog = show;
        }
    }

    fn draw_cleanup_tab(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔍 扫描系统").clicked() {
                self.perform_scan();
            }

            if self.scan_in_progress {
                ui.add(egui::ProgressBar::new(self.scan_progress / 100.0)
                    .text(format!("扫描中... {:.1}%", self.scan_progress)));
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.add(Button::new(RichText::new("🗑 清理选中项").color(Color32::WHITE))
                    .fill(Color32::from_rgb(239, 68, 68)))
                    .clicked()
                {
                    if self.selected_count > 0 {
                        self.show_confirm_dialog = true;
                    }
                }

                ui.label(RichText::new(format!("已选: {} ({})", self.selected_count, Self::format_size(self.total_size)))
                    .strong());
            });
        });

        ui.separator();

        ScrollArea::vertical().show(ui, |ui| {
            self.draw_categories(ui);
        });
    }

    fn draw_logs_tab(&mut self, ui: &mut Ui) {
        ui.heading("清理日志");
        ui.separator();

        ScrollArea::vertical().show(ui, |ui| {
            if self.logs.is_empty() {
                ui.label("暂无日志记录");
            } else {
                for log in &self.logs {
                    let level_color = match log.level {
                        LogLevel::Info => Color32::from_rgb(59, 130, 246),
                        LogLevel::Warning => Color32::from_rgb(234, 179, 8),
                        LogLevel::Error => Color32::from_rgb(239, 68, 68),
                    };

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&log.timestamp).monospace().size(11.0));
                        ui.colored_label(level_color, format!("[{:?}]", log.level));
                        ui.label(&log.message);
                    });
                }
            }
        });
    }

    fn draw_settings_tab(&mut self, ui: &mut Ui) {
        ui.heading("设置");
        ui.separator();

        ui.group(|ui| {
            ui.label("安全设置");
            ui.horizontal(|ui| {
                ui.label("覆写次数:");
                ui.add(egui::Slider::new(&mut self.overwrite_passes, 1..=35));
            });
            ui.label("覆写次数越多，数据恢复难度越大，但清理时间也会更长。");
        });

        ui.add_space(16.0);

        ui.group(|ui| {
            ui.label("外观设置");
            ui.checkbox(&mut self.dark_mode, "深色模式");
        });

        ui.add_space(16.0);

        ui.group(|ui| {
            ui.label("关于");
            ui.label("Resignation Delete v0.1.0");
            ui.label("一个用于清理离职前敏感数据的工具");
        });
    }
}

impl App for ResignationApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("🧹 Resignation Delete").size(24.0).strong());
                ui.separator();

                let tab_text_size = 16.0;
                let cleanup_text = if self.current_tab == Tab::Cleanup {
                    RichText::new("🧹 清理").strong().size(tab_text_size)
                } else {
                    RichText::new("🧹 清理").size(tab_text_size)
                };
                if ui.selectable_label(self.current_tab == Tab::Cleanup, cleanup_text).clicked() {
                    self.current_tab = Tab::Cleanup;
                }

                let logs_text = if self.current_tab == Tab::Logs {
                    RichText::new("📋 日志").strong().size(tab_text_size)
                } else {
                    RichText::new("📋 日志").size(tab_text_size)
                };
                if ui.selectable_label(self.current_tab == Tab::Logs, logs_text).clicked() {
                    self.current_tab = Tab::Logs;
                }

                let settings_text = if self.current_tab == Tab::Settings {
                    RichText::new("⚙️ 设置").strong().size(tab_text_size)
                } else {
                    RichText::new("⚙️ 设置").size(tab_text_size)
                };
                if ui.selectable_label(self.current_tab == Tab::Settings, settings_text).clicked() {
                    self.current_tab = Tab::Settings;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Cleanup => self.draw_cleanup_tab(ui),
                Tab::Logs => self.draw_logs_tab(ui),
                Tab::Settings => self.draw_settings_tab(ui),
            }
        });

        self.draw_confirm_dialog(ctx);
        self.draw_result_dialog(ctx);
    }
}

fn setup_custom_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.window_margin = Margin::same(12.0);

    style.visuals.widgets.active.rounding = Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
    style.visuals.widgets.open.rounding = Rounding::same(8.0);
    style.visuals.window_rounding = Rounding::same(12.0);
    style.visuals.window_shadow = egui::epaint::Shadow {
        offset: Vec2::new(0.0, 8.0),
        blur: 16.0,
        spread: 0.0,
        color: Color32::from_black_alpha(64),
    };
    style.visuals.popup_shadow = egui::epaint::Shadow {
        offset: Vec2::new(0.0, 4.0),
        blur: 8.0,
        spread: 0.0,
        color: Color32::from_black_alpha(48),
    };
    style.visuals.collapsing_header_frame = true;
    style.visuals.indent_has_left_vline = true;

    ctx.set_style(style);
}

fn load_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    let font_path = std::path::Path::new("../assets/NotoSansCJKsc-Regular.otf");
    if let Ok(font_data) = std::fs::read(font_path) {
        fonts.font_data.insert(
            "NotoSansCJKsc".to_owned(),
            FontData::from_owned(font_data),
        );
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "NotoSansCJKsc".to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("NotoSansCJKsc".to_owned());
    }

    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    run_native(
        "Resignation Delete",
        options,
        Box::new(|cc| {
            load_custom_fonts(&cc.egui_ctx);
            setup_custom_style(&cc.egui_ctx);
            Ok(Box::new(ResignationApp::new(cc)))
        }),
    )
}
