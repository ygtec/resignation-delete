use eframe::{egui, App, CreationContext, Frame, NativeOptions, run_native};
use egui::{Color32, Context, FontData, FontDefinitions, FontFamily, RichText, Rounding, Stroke, Style, Vec2, Visuals, Window, Button, ScrollArea, Separator, Ui, Layout, Align, Margin, TextWrapMode};
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
    pub icon: String,
    pub color: Color32,
    pub items: Vec<CategoryItem>,
    pub expanded: bool,
}

pub struct ResignationApp {
    current_tab: Tab,
    categories: Vec<Category>,
    cleaner: Cleaner,
    tasks: Vec<CleanTask>,
    is_scanning: bool,
    scan_progress: f64,
    scan_complete: bool,
    total_selected_size: u64,
    total_item_count: usize,
    selected_item_count: usize,
    show_confirm_dialog: bool,
    show_success_dialog: bool,
    success_count: usize,
    fail_count: usize,
    cleanup_started: bool,
}

impl ResignationApp {
    pub fn new(cc: &CreationContext) -> Self {
        Self {
            current_tab: Tab::Cleanup,
            categories: Self::create_default_categories(),
            cleaner: Cleaner::new(),
            tasks: vec![],
            is_scanning: false,
            scan_progress: 0.0,
            scan_complete: false,
            total_selected_size: 0,
            total_item_count: 0,
            selected_item_count: 0,
            show_confirm_dialog: false,
            show_success_dialog: false,
            success_count: 0,
            fail_count: 0,
            cleanup_started: false,
        }
    }

    fn create_default_categories() -> Vec<Category> {
        vec![
            Self::create_dev_tools_category(),
            Self::create_browsers_category(),
            Self::create_ai_tools_category(),
            Self::create_office_category(),
        ]
    }

    fn create_dev_tools_category() -> Category {
        let items = vec![
            CategoryItem::new("vscode", "VSCode", "Visual Studio Code 用户数据", "~/.vscode/", 50_000_000, RiskLevel::High),
            CategoryItem::new("intellij", "IntelliJ IDEA", "JetBrains IDE 用户数据", "~/.config/JetBrains/IntelliJIdea/", 100_000_000, RiskLevel::High),
            CategoryItem::new("pycharm", "PyCharm", "Python IDE 用户数据", "~/.config/JetBrains/PyCharm/", 75_000_000, RiskLevel::High),
            CategoryItem::new("webstorm", "WebStorm", "Web IDE 用户数据", "~/.config/JetBrains/WebStorm/", 60_000_000, RiskLevel::High),
            CategoryItem::new("vscode-ext", "VSCode 扩展", "VSCode 扩展数据", "~/.vscode/extensions/", 150_000_000, RiskLevel::Medium),
            CategoryItem::new("node-modules", "Node Modules", "Node.js 依赖", "~/node_modules/", 200_000_000, RiskLevel::Low),
            CategoryItem::new("npm-cache", "npm Cache", "npm 缓存", "~/.npm/", 50_000_000, RiskLevel::Low),
            CategoryItem::new("yarn-cache", "yarn Cache", "yarn 缓存", "~/.cache/yarn/", 40_000_000, RiskLevel::Low),
            CategoryItem::new("pnpm-cache", "pnpm Cache", "pnpm 缓存", "~/.local/share/pnpm/", 60_000_000, RiskLevel::Low),
            CategoryItem::new("cargo-cache", "Cargo Cache", "Rust 依赖缓存", "~/.cargo/registry/", 300_000_000, RiskLevel::Low),
            CategoryItem::new("rustup", "Rustup", "Rust 工具链", "~/.rustup/", 500_000_000, RiskLevel::Low),
            CategoryItem::new("go-modules", "Go Modules", "Go 依赖", "~/go/pkg/mod/", 150_000_000, RiskLevel::Low),
            CategoryItem::new("gradle-cache", "Gradle Cache", "Gradle 缓存", "~/.gradle/caches/", 100_000_000, RiskLevel::Low),
            CategoryItem::new("maven-cache", "Maven Cache", "Maven 仓库", "~/.m2/repository/", 75_000_000, RiskLevel::Low),
            CategoryItem::new("docker", "Docker", "Docker 镜像和容器", "~/.docker/", 500_000_000, RiskLevel::Medium),
            CategoryItem::new("git-config", "Git Config", "Git 全局配置", "~/.gitconfig", 4_096, RiskLevel::High),
            CategoryItem::new("ssh-keys", "SSH Keys", "SSH 私钥", "~/.ssh/", 16_384, RiskLevel::Critical),
            CategoryItem::new("gnupg", "GNUPG", "GPG 密钥", "~/.gnupg/", 32_768, RiskLevel::High),
            CategoryItem::new("aws-cli", "AWS CLI", "AWS 凭证", "~/.aws/", 8_192, RiskLevel::Critical),
            CategoryItem::new("google-cloud", "Google Cloud", "GCP 凭证", "~/.config/gcloud/", 16_384, RiskLevel::Critical),
            CategoryItem::new("azure-cli", "Azure CLI", "Azure 凭证", "~/.azure/", 8_192, RiskLevel::Critical),
            CategoryItem::new("docker-config", "Docker Config", "Docker 配置", "~/.docker/config.json", 4_096, RiskLevel::High),
            CategoryItem::new("kube-config", "Kubernetes", "K8s 配置", "~/.kube/config", 4_096, RiskLevel::High),
            CategoryItem::new("terraform", "Terraform", "TF 状态文件", "~/.terraform.d/", 8_192, RiskLevel::Medium),
            CategoryItem::new("github-desktop", "GitHub Desktop", "GitHub Desktop 数据", "~/AppData/Local/GitHubDesktop/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("sourcetree", "SourceTree", "SourceTree 配置", "~/AppData/Local/Atlassian/SourceTree/", 20_000_000, RiskLevel::Medium),
            CategoryItem::new("gitkraken", "GitKraken", "GitKraken 配置", "~/AppData/Local/gitkraken/", 30_000_000, RiskLevel::Medium),
            CategoryItem::new("android-studio", "Android Studio", "Android Studio 配置", "~/.android/", 100_000_000, RiskLevel::High),
            CategoryItem::new("xcode", "Xcode", "Xcode 派生数据", "~/Library/Developer/Xcode/DerivedData/", 500_000_000, RiskLevel::Medium),
            CategoryItem::new("cocoapods", "CocoaPods", "CocoaPods 缓存", "~/.cocoapods/", 200_000_000, RiskLevel::Low),
            CategoryItem::new("ruby-gems", "Ruby Gems", "Ruby 依赖", "~/.gem/", 50_000_000, RiskLevel::Low),
            CategoryItem::new("composer", "Composer", "PHP 依赖", "~/.composer/", 30_000_000, RiskLevel::Low),
        ];
        Category {
            name: "开发工具".to_string(),
            icon: "💻".to_string(),
            color: Color32::from_rgb(0, 122, 204),
            items,
            expanded: true,
        }
    }

    fn create_browsers_category() -> Category {
        let items = vec![
            CategoryItem::new("chrome", "Chrome", "Google Chrome 浏览器数据", "~/AppData/Local/Google/Chrome/", 200_000_000, RiskLevel::High),
            CategoryItem::new("edge", "Edge", "Microsoft Edge 浏览器数据", "~/AppData/Local/Microsoft/Edge/", 150_000_000, RiskLevel::High),
            CategoryItem::new("firefox", "Firefox", "Mozilla Firefox 浏览器数据", "~/AppData/Roaming/Mozilla/Firefox/", 100_000_000, RiskLevel::High),
            CategoryItem::new("safari", "Safari", "Safari 浏览器数据", "~/Library/Safari/", 100_000_000, RiskLevel::High),
            CategoryItem::new("opera", "Opera", "Opera 浏览器数据", "~/AppData/Roaming/Opera Software/", 80_000_000, RiskLevel::High),
            CategoryItem::new("brave", "Brave", "Brave 浏览器数据", "~/AppData/Local/BraveSoftware/", 80_000_000, RiskLevel::High),
            CategoryItem::new("vivaldi", "Vivaldi", "Vivaldi 浏览器数据", "~/AppData/Local/Vivaldi/", 80_000_000, RiskLevel::High),
            CategoryItem::new("qq-browser", "QQ浏览器", "QQ浏览器数据", "~/AppData/Local/Tencent/QQBrowser/", 100_000_000, RiskLevel::High),
            CategoryItem::new("360-browser", "360安全浏览器", "360浏览器数据", "~/AppData/Local/360Chrome/", 100_000_000, RiskLevel::High),
            CategoryItem::new("360-speed", "360极速浏览器", "360极速浏览器数据", "~/AppData/Local/360se6/", 100_000_000, RiskLevel::High),
            CategoryItem::new("sogou", "搜狗浏览器", "搜狗浏览器数据", "~/AppData/Roaming/SogouExplorer/", 80_000_000, RiskLevel::High),
            CategoryItem::new("uc", "UC浏览器", "UC浏览器数据", "~/AppData/Local/UCBrowser/", 80_000_000, RiskLevel::High),
            CategoryItem::new("maxthon", "傲游浏览器", "傲游浏览器数据", "~/AppData/Roaming/Maxthon5/", 80_000_000, RiskLevel::High),
            CategoryItem::new("cent", "百分浏览器", "CentBrowser数据", "~/AppData/Local/CentBrowser/", 80_000_000, RiskLevel::High),
            CategoryItem::new("yandex", "Yandex", "Yandex浏览器数据", "~/AppData/Local/Yandex/", 80_000_000, RiskLevel::High),
            CategoryItem::new("tor", "Tor Browser", "Tor浏览器数据", "~/AppData/Roaming/Tor Browser/", 50_000_000, RiskLevel::Critical),
            CategoryItem::new("waterfox", "Waterfox", "Waterfox浏览器数据", "~/AppData/Roaming/Waterfox/", 80_000_000, RiskLevel::High),
            CategoryItem::new("pale-moon", "Pale Moon", "Pale Moon浏览器数据", "~/AppData/Roaming/Pale Moon/", 60_000_000, RiskLevel::High),
            CategoryItem::new("seamonkey", "SeaMonkey", "SeaMonkey浏览器数据", "~/AppData/Roaming/Mozilla/SeaMonkey/", 60_000_000, RiskLevel::High),
            CategoryItem::new("thor", "雷神浏览器", "跨境电商浏览器", "~/AppData/Local/ThorBrowser/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("adspower", "AdsPower", "AdsPower指纹浏览器", "~/AppData/Local/AdsPower/", 150_000_000, RiskLevel::Critical),
            CategoryItem::new("multilogin", "Multilogin", "Multilogin指纹浏览器", "~/AppData/Local/Multilogin/", 150_000_000, RiskLevel::Critical),
            CategoryItem::new("lalicat", "拉力猫", "拉力猫指纹浏览器", "~/AppData/Local/Lalicat/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("ixbrowser", "ixBrowser", "ixBrowser指纹浏览器", "~/AppData/Local/ixBrowser/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("vmlogin", "VMLogin", "VMLogin指纹浏览器", "~/AppData/Local/VMLogin/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("hubstudio", "HubStudio", "HubStudio指纹浏览器", "~/AppData/Local/HubStudio/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("dolphin", "Dolphin", "Dolphin指纹浏览器", "~/AppData/Local/Dolphin/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("morelogin", "MoreLogin", "MoreLogin指纹浏览器", "~/AppData/Local/MoreLogin/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("gologin", "GoLogin", "GoLogin指纹浏览器", "~/AppData/Local/GoLogin/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("incogniton", "Incogniton", "Incogniton指纹浏览器", "~/AppData/Local/Incogniton/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("clonbrowser", "ClonBrowser", "ClonBrowser指纹浏览器", "~/AppData/Local/ClonBrowser/", 100_000_000, RiskLevel::Critical),
            CategoryItem::new("octobrowser", "Octo Browser", "OctoBrowser指纹浏览器", "~/AppData/Local/OctoBrowser/", 100_000_000, RiskLevel::Critical),
        ];
        Category {
            name: "浏览器".to_string(),
            icon: "🌐".to_string(),
            color: Color32::from_rgb(0, 188, 212),
            items,
            expanded: true,
        }
    }

    fn create_ai_tools_category() -> Category {
        let items = vec![
            CategoryItem::new("cursor", "Cursor", "Cursor AI 编辑器", "~/AppData/Roaming/Cursor/", 50_000_000, RiskLevel::High),
            CategoryItem::new("claude", "Claude Desktop", "Claude 桌面应用", "~/AppData/Roaming/Claude/", 30_000_000, RiskLevel::High),
            CategoryItem::new("github-copilot", "GitHub Copilot", "Copilot 扩展", "~/.config/github-copilot/", 10_000_000, RiskLevel::High),
            CategoryItem::new("kimi", "Kimi", "Kimi AI 助手", "~/AppData/Roaming/Kimi/", 20_000_000, RiskLevel::Medium),
            CategoryItem::new("qwen", "通义千问", "通义千问配置", "~/AppData/Roaming/Qwen/", 20_000_000, RiskLevel::Medium),
            CategoryItem::new("opencode", "OpenCode", "OpenCode AI编程助手", "~/AppData/Roaming/OpenCode/", 30_000_000, RiskLevel::High),
            CategoryItem::new("trae", "Trae", "Trae AI 编辑器", "~/AppData/Roaming/Trae/", 30_000_000, RiskLevel::High),
            CategoryItem::new("chatgpt", "ChatGPT Desktop", "ChatGPT桌面应用", "~/AppData/Roaming/ChatGPT/", 30_000_000, RiskLevel::High),
            CategoryItem::new("perplexity", "Perplexity", "Perplexity AI配置", "~/AppData/Roaming/Perplexity/", 10_000_000, RiskLevel::Medium),
            CategoryItem::new("notion-ai", "Notion AI", "Notion AI配置", "~/AppData/Roaming/Notion/", 20_000_000, RiskLevel::Medium),
            CategoryItem::new("midjourney", "Midjourney", "Midjourney配置", "~/AppData/Roaming/Midjourney/", 10_000_000, RiskLevel::Low),
            CategoryItem::new("stable-diffusion", "Stable Diffusion", "SD本地模型", "~/stable-diffusion/", 2_000_000_000, RiskLevel::Low),
            CategoryItem::new("comfyui", "ComfyUI", "ComfyUI工作流", "~/ComfyUI/", 500_000_000, RiskLevel::Low),
            CategoryItem::new("fooocus", "Fooocus", "Fooocus配置", "~/Fooocus/", 500_000_000, RiskLevel::Low),
            CategoryItem::new("ollama", "Ollama", "Ollama本地模型", "~/.ollama/", 5_000_000_000, RiskLevel::Low),
            CategoryItem::new("lm-studio", "LM Studio", "LM Studio配置", "~/AppData/Roaming/LM Studio/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("gpt4all", "GPT4All", "GPT4All模型", "~/AppData/Roaming/GPT4All/", 500_000_000, RiskLevel::Low),
            CategoryItem::new("jan", "Jan", "Jan AI运行器", "~/AppData/Roaming/Jan/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("anythingllm", "AnythingLLM", "AnythingLLM知识库", "~/AppData/Roaming/AnythingLLM/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("continue", "Continue", "Continue AI编程", "~/.continue/", 10_000_000, RiskLevel::High),
            CategoryItem::new("tabnine", "Tabnine", "Tabnine代码补全", "~/.tabnine/", 10_000_000, RiskLevel::Medium),
            CategoryItem::new("codeium", "Codeium", "Codeium代码补全", "~/.codeium/", 10_000_000, RiskLevel::Medium),
            CategoryItem::new("codewhisperer", "CodeWhisperer", "Amazon代码补全", "~/.aws/codewhisperer/", 5_000_000, RiskLevel::Medium),
            CategoryItem::new("replit", "Replit", "Replit在线IDE", "~/AppData/Roaming/Replit/", 10_000_000, RiskLevel::Low),
            CategoryItem::new("codepen", "CodePen", "CodePen编辑器", "~/AppData/Roaming/CodePen/", 10_000_000, RiskLevel::Low),
            CategoryItem::new("stackblitz", "StackBlitz", "StackBlitz IDE", "~/AppData/Roaming/StackBlitz/", 10_000_000, RiskLevel::Low),
            CategoryItem::new("codesandbox", "CodeSandbox", "CodeSandbox IDE", "~/AppData/Roaming/CodeSandbox/", 10_000_000, RiskLevel::Low),
            CategoryItem::new("huggingface", "Hugging Face", "HF模型和令牌", "~/.huggingface/", 1_000_000_000, RiskLevel::High),
            CategoryItem::new("openai-api", "OpenAI API", "OpenAI API密钥", "~/.openai/", 4_096, RiskLevel::Critical),
            CategoryItem::new("anthropic-api", "Anthropic API", "Anthropic API密钥", "~/.anthropic/", 4_096, RiskLevel::Critical),
            CategoryItem::new("google-ai", "Google AI", "Google AI Studio", "~/.google-ai/", 4_096, RiskLevel::Critical),
            CategoryItem::new("cohere", "Cohere", "Cohere API配置", "~/.cohere/", 4_096, RiskLevel::Critical),
        ];
        Category {
            name: "AI 工具".to_string(),
            icon: "🤖".to_string(),
            color: Color32::from_rgb(156, 39, 176),
            items,
            expanded: true,
        }
    }

    fn create_office_category() -> Category {
        let items = vec![
            CategoryItem::new("wps", "WPS Office", "WPS用户数据", "~/AppData/Local/Kingsoft/", 100_000_000, RiskLevel::High),
            CategoryItem::new("wechat", "微信", "微信聊天记录", "~/Documents/WeChat Files/", 500_000_000, RiskLevel::Critical),
            CategoryItem::new("qq", "QQ", "QQ聊天记录", "~/Documents/Tencent Files/", 300_000_000, RiskLevel::Critical),
            CategoryItem::new("dingtalk", "钉钉", "钉钉聊天记录", "~/AppData/Roaming/DingTalk/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("feishu", "飞书", "飞书聊天记录", "~/AppData/Roaming/Feishu/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("lark", "Lark", "Lark聊天记录", "~/AppData/Roaming/Lark/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("office", "Microsoft Office", "Office缓存", "~/AppData/Local/Microsoft/Office/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("wxwork", "企业微信", "企业微信数据", "~/Documents/WXWork/", 300_000_000, RiskLevel::Critical),
            CategoryItem::new("tim", "TIM", "TIM聊天记录", "~/Documents/TIM/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("rtx", "腾讯通RTX", "RTX聊天记录", "~/AppData/Roaming/RTXC/", 100_000_000, RiskLevel::High),
            CategoryItem::new("teams", "Microsoft Teams", "Teams聊天记录", "~/AppData/Roaming/Microsoft/Teams/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("slack", "Slack", "Slack聊天记录", "~/AppData/Roaming/Slack/", 150_000_000, RiskLevel::High),
            CategoryItem::new("discord", "Discord", "Discord缓存", "~/AppData/Roaming/discord/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("zoom", "Zoom", "Zoom会议记录", "~/AppData/Roaming/Zoom/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("tencent-meeting", "腾讯会议", "腾讯会议记录", "~/AppData/Roaming/Tencent/WeMeet/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("dingtalk-meeting", "钉钉会议", "钉钉会议记录", "~/AppData/Roaming/DingTalk/Meeting/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("skype", "Skype", "Skype聊天记录", "~/AppData/Roaming/Skype/", 100_000_000, RiskLevel::High),
            CategoryItem::new("telegram", "Telegram", "Telegram聊天记录", "~/AppData/Roaming/Telegram Desktop/", 200_000_000, RiskLevel::High),
            CategoryItem::new("whatsapp", "WhatsApp", "WhatsApp备份", "~/AppData/Roaming/WhatsApp/", 200_000_000, RiskLevel::High),
            CategoryItem::new("line", "LINE", "LINE聊天记录", "~/AppData/Roaming/LINE/", 150_000_000, RiskLevel::High),
            CategoryItem::new("evernote", "印象笔记", "印象笔记数据", "~/AppData/Roaming/Evernote/", 100_000_000, RiskLevel::High),
            CategoryItem::new("yuque", "语雀", "语雀文档", "~/AppData/Roaming/Yuque/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("shimo", "石墨文档", "石墨文档", "~/AppData/Roaming/Shimo/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("tencent-docs", "腾讯文档", "腾讯文档", "~/AppData/Roaming/TencentDocs/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("baidu-cloud", "百度网盘", "百度网盘配置", "~/AppData/Roaming/baidu/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("aliyun-drive", "阿里云盘", "阿里云盘配置", "~/AppData/Local/aDrive/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("onedrive", "OneDrive", "OneDrive同步", "~/AppData/Local/Microsoft/OneDrive/", 100_000_000, RiskLevel::Medium),
            CategoryItem::new("google-drive", "Google Drive", "Google Drive", "~/AppData/Local/Google/Drive/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("dropbox", "Dropbox", "Dropbox同步", "~/AppData/Roaming/Dropbox/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("jianguoyun", "坚果云", "坚果云同步", "~/AppData/Roaming/Nutstore/", 50_000_000, RiskLevel::Medium),
            CategoryItem::new("tianyi", "天翼云盘", "天翼云盘", "~/AppData/Roaming/21cn/", 30_000_000, RiskLevel::Medium),
            CategoryItem::new("hecaiyun", "和彩云", "和彩云", "~/AppData/Roaming/139/", 30_000_000, RiskLevel::Medium),
        ];
        Category {
            name: "办公软件".to_string(),
            icon: "📊".to_string(),
            color: Color32::from_rgb(76, 175, 80),
            items,
            expanded: true,
        }
    }

    fn format_size(size: u64) -> String {
        if size >= 1_000_000_000 {
            format!("{:.2} GB", size as f64 / 1_000_000_000.0)
        } else if size >= 1_000_000 {
            format!("{:.2} MB", size as f64 / 1_000_000.0)
        } else if size >= 1_000 {
            format!("{:.2} KB", size as f64 / 1_000.0)
        } else {
            format!("{} B", size)
        }
    }

    fn get_risk_label(risk: &RiskLevel) -> &'static str {
        match risk {
            RiskLevel::Critical => "严重",
            RiskLevel::High => "高风险",
            RiskLevel::Medium => "中等",
            RiskLevel::Low => "低风险",
        }
    }

    fn get_risk_color(risk: &RiskLevel) -> Color32 {
        match risk {
            RiskLevel::Critical => Color32::from_rgb(255, 50, 50),
            RiskLevel::High => Color32::from_rgb(255, 140, 0),
            RiskLevel::Medium => Color32::from_rgb(255, 200, 0),
            RiskLevel::Low => Color32::from_rgb(80, 180, 80),
        }
    }

    fn update_counts(&mut self) {
        self.total_item_count = self.categories.iter().map(|c| c.items.len()).sum();
        self.selected_item_count = self.categories.iter()
            .flat_map(|c| &c.items)
            .filter(|item| item.selected)
            .count();
        self.total_selected_size = self.categories.iter()
            .flat_map(|c| &c.items)
            .filter(|item| item.selected)
            .map(|item| item.size)
            .sum();
    }

    fn select_all_in_category(&mut self, cat_idx: usize, select: bool) {
        for item in &mut self.categories[cat_idx].items {
            item.selected = select;
        }
        self.update_counts();
    }

    fn select_all(&mut self, select: bool) {
        for cat in &mut self.categories {
            for item in &mut cat.items {
                item.selected = select;
            }
        }
        self.update_counts();
    }

    fn start_scan(&mut self) {
        self.is_scanning = true;
        self.scan_progress = 0.0;
        self.scan_complete = false;
        
        let scanners: Vec<Box<dyn Scanner>> = vec![
            Box::new(GitSshScanner),
            Box::new(BrowsersScanner),
            Box::new(JetBrainsScanner),
            Box::new(VSCodeScanner),
            Box::new(AIToolsScanner),
        ];

        for cat in &mut self.categories {
            for item in &mut cat.items {
                item.scanned = false;
                item.detected = false;
                item.size = 0;
            }
        }

        let total_scanners = scanners.len();
        for (i, scanner) in scanners.into_iter().enumerate() {
            let items = scanner.scan();
            for scanned_item in items {
                for cat in &mut self.categories {
                    for item in &mut cat.items {
                        if item.id == scanned_item.id || item.path.contains(&scanned_item.path) || scanned_item.path.contains(&item.path) {
                            item.scanned = true;
                            item.detected = scanned_item.size > 0;
                            if scanned_item.size > 0 {
                                item.size = scanned_item.size;
                            }
                        }
                    }
                }
            }
            self.scan_progress = (i + 1) as f64 / total_scanners as f64;
        }

        self.is_scanning = false;
        self.scan_complete = true;
        self.update_counts();
    }

    fn start_cleanup(&mut self) {
        self.tasks.clear();
        self.success_count = 0;
        self.fail_count = 0;
        self.cleanup_started = true;

        for cat in &self.categories {
            for item in &cat.items {
                if item.selected {
                    self.tasks.push(CleanTask::new(
                        item.id.clone(),
                        item.name.clone(),
                        item.path.clone(),
                        item.size,
                    ));
                }
            }
        }
    }

    fn execute_cleanup(&mut self, ctx: &Context) {
        let mut data_items = Vec::new();
        for cat in &self.categories {
            for item in &cat.items {
                if item.selected {
                    data_items.push(item.to_data_item());
                }
            }
        }

        if data_items.is_empty() {
            return;
        }

        self.cleaner.clear_tasks();
        self.cleaner.add_tasks(data_items);

        // clean_all 的回调用于确认是否继续清理
        let _ = self.cleaner.clean_all(|_| true);

        // 统计清理结果
        let mut success = 0usize;
        let mut fail = 0usize;
        for task in self.cleaner.tasks() {
            match &task.status {
                CleanStatus::Completed => success += 1,
                CleanStatus::Failed(_) => fail += 1,
                _ => {}
            }
        }

        self.success_count = success;
        self.fail_count = fail;
        self.show_success_dialog = true;

        for cat in &mut self.categories {
            for item in &mut cat.items {
                if item.selected {
                    item.selected = false;
                }
            }
        }
        self.update_counts();
    }
}

impl App for ResignationApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🗑️ Resignation Delete - 离职数据清理工具");
                ui.separator();
                ui.label(RichText::new("一键清除离职电脑上的个人数据").color(Color32::GRAY).size(12.0));
            });
            Separator::default().ui(ui);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Cleanup, " 清理");
                ui.selectable_value(&mut self.current_tab, Tab::Logs, " 日志");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "️ 设置");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Cleanup => self.draw_cleanup_panel(ui),
                Tab::Logs => self.draw_logs_panel(ui),
                Tab::Settings => self.draw_settings_panel(ui),
            }
        });

        if self.show_confirm_dialog {
            self.draw_confirm_dialog(ctx);
        }

        if self.show_success_dialog {
            self.draw_success_dialog(ctx);
        }
    }
}

impl ResignationApp {
    fn draw_cleanup_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // 左侧操作面板
            ui.vertical(|ui| {
                ui.set_width(200.0);
                self.draw_control_panel(ui);
            });
            
            ui.separator();
            
            // 右侧分类列表
            ui.vertical(|ui| {
                self.draw_categories(ui);
            });
        });
    }

    fn draw_control_panel(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.set_width(180.0);
            ui.heading("操作面板");
            Separator::default().ui(ui);
            
            // 扫描按钮
            let scan_btn = Button::new(
                RichText::new("🔍 一键扫描").size(16.0)
            )
            .fill(Color32::from_rgb(33, 150, 243))
            .stroke(Stroke::NONE)
            .rounding(8.0)
            .min_size(Vec2::new(160.0, 45.0));
            
            if ui.add_enabled(!self.is_scanning, scan_btn).clicked() {
                self.start_scan();
            }
            
            ui.add_space(8.0);
            
            // 清理按钮
            let has_selected = self.selected_item_count > 0;
            let clean_btn = Button::new(
                RichText::new("🗑️ 一键清理").size(16.0)
            )
            .fill(if has_selected { Color32::from_rgb(244, 67, 54) } else { Color32::GRAY })
            .stroke(Stroke::NONE)
            .rounding(8.0)
            .min_size(Vec2::new(160.0, 45.0));
            
            if ui.add_enabled(has_selected, clean_btn).clicked() {
                self.show_confirm_dialog = true;
            }
            
            ui.add_space(16.0);
            
            // 统计信息
            ui.group(|ui| {
                ui.heading("统计信息");
                Separator::default().ui(ui);
                ui.label(format!("总项目: {}", self.total_item_count));
                ui.label(format!("已选择: {} 项", self.selected_item_count));
                ui.label(format!("总大小: {}", Self::format_size(self.total_selected_size)));
                if self.scan_complete {
                    ui.label(RichText::new("✅ 扫描完成").color(Color32::GREEN));
                }
            });
            
            ui.add_space(8.0);
            
            // 全选/取消全选
            ui.horizontal(|ui| {
                if ui.button(RichText::new("全选").size(13.0)).clicked() {
                    self.select_all(true);
                }
                if ui.button(RichText::new("取消全选").size(13.0)).clicked() {
                    self.select_all(false);
                }
            });
        });
    }

    fn draw_categories(&mut self, ui: &mut Ui) {
        ui.heading("可清理项目");
        Separator::default().ui(ui);
        
        ScrollArea::vertical().show(ui, |ui| {
            for cat_idx in 0..self.categories.len() {
                let cat = &self.categories[cat_idx];
                let selected = cat.items.iter().filter(|i| i.selected).count();
                let total = cat.items.len();
                
                let header = egui::CollapsingHeader::new(
                    RichText::new(format!("{} {} ({}/{})", cat.icon, cat.name, selected, total))
                        .color(cat.color)
                        .size(16.0)
                        .strong()
                )
                .default_open(cat.expanded)
                .show(ui, |ui| {
                    // 分类全选按钮
                    let all_selected = selected == total;
                    ui.horizontal(|ui| {
                        let mut check = all_selected;
                        if ui.checkbox(&mut check, RichText::new("全选此分类").color(cat.color)).changed() {
                            self.select_all_in_category(cat_idx, check);
                        }
                    });
                    
                    ui.add_space(4.0);
                    
                    // 项目列表
                    for item_idx in 0..cat.items.len() {
                        let item = &cat.items[item_idx];
                        
                        let bg_color = if item.selected {
                            Color32::from_rgb(240, 248, 255)
                        } else {
                            Color32::TRANSPARENT
                        };
                        
                        egui::Frame::none()
                            .fill(bg_color)
                            .rounding(4.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let mut checked = item.selected;
                                    if ui.checkbox(&mut checked, "").changed() {
                                        self.categories[cat_idx].items[item_idx].selected = checked;
                                        self.update_counts();
                                    }
                                    
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&item.name).strong().size(14.0));
                                            if !item.description.is_empty() {
                                                ui.label(RichText::new(&item.description).color(Color32::GRAY).size(12.0));
                                            }
                                        });
                                        
                                        ui.horizontal(|ui| {
                                            let risk_color = Self::get_risk_color(&item.risk_level);
                                            let risk_label = Self::get_risk_label(&item.risk_level);
                                            ui.label(RichText::new(format!("[{}]", risk_label)).color(risk_color).size(11.0));
                                            
                                            ui.label(RichText::new(&item.path).color(Color32::GRAY).size(11.0));
                                            
                                            if item.size > 0 {
                                                ui.label(RichText::new(Self::format_size(item.size)).color(Color32::GRAY).size(11.0));
                                            }
                                            
                                            if item.detected {
                                                ui.label(RichText::new("✅ 已发现").color(Color32::GREEN).size(11.0));
                                            }
                                        });
                                    });
                                });
                            });
                        
                        ui.add_space(2.0);
                        Separator::default().ui(ui);
                    }
                });
                
                self.categories[cat_idx].expanded = header.header_response.fully_open();
                ui.add_space(8.0);
            }
        });
    }

    fn draw_confirm_dialog(&mut self, ctx: &Context) {
        Window::new("确认清理")
            .collapsible(false)
            .resizable(false)
            .default_width(500.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("⚠️ 确认要清理以下数据吗？").size(16.0).strong());
                ui.add_space(8.0);
                ui.label(format!("共 {} 个项目，总大小 {}", self.selected_item_count, Self::format_size(self.total_selected_size)));
                ui.add_space(8.0);
                
                ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    for cat in &self.categories {
                        for item in &cat.items {
                            if item.selected {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{} - {}", item.name, item.path));
                                });
                            }
                        }
                    }
                });
                
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button("确认清理").clicked() {
                        self.start_cleanup();
                        self.show_confirm_dialog = false;
                        self.execute_cleanup(ctx);
                    }
                    if ui.button("取消").clicked() {
                        self.show_confirm_dialog = false;
                    }
                });
            });
    }

    fn draw_success_dialog(&mut self, ctx: &Context) {
        Window::new("清理结果")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("✅ 清理完成！").size(18.0).strong().color(Color32::GREEN));
                ui.add_space(12.0);
                ui.label(format!("成功清理: {} 个项目", self.success_count));
                ui.label(format!("清理失败: {} 个项目", self.fail_count));
                ui.add_space(16.0);
                if ui.button("确定").clicked() {
                    self.show_success_dialog = false;
                }
            });
    }

    fn draw_logs_panel(&mut self, ui: &mut Ui) {
        ui.heading("操作日志");
        Separator::default().ui(ui);
        
        ScrollArea::vertical().show(ui, |ui| {
            let logs = self.cleaner.get_logs();
            if logs.is_empty() {
                ui.label(RichText::new("暂无日志记录").color(Color32::GRAY));
            } else {
                for log in logs {
                    let color = match log.level {
                        LogLevel::Info => Color32::GREEN,
                        LogLevel::Warning => Color32::YELLOW,
                        LogLevel::Error => Color32::RED,
                        LogLevel::Debug => Color32::GRAY,
                    };
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("[{}]", log.timestamp.format("%H:%M:%S"))).color(Color32::GRAY).size(12.0));
                        ui.label(RichText::new(format!("[{}]", match log.level {
                            LogLevel::Info => "INFO",
                            LogLevel::Warning => "WARN",
                            LogLevel::Error => "ERROR",
                            LogLevel::Debug => "DEBUG",
                        })).color(color).size(12.0));
                        ui.label(RichText::new(&log.message).size(12.0));
                    });
                }
            }
        });
    }

    fn draw_settings_panel(&mut self, ui: &mut Ui) {
        ui.heading("设置");
        Separator::default().ui(ui);
        
        ui.group(|ui| {
            ui.heading("清理选项");
            ui.checkbox(&mut true, "清理后覆写文件（安全删除）");
            ui.checkbox(&mut true, "显示清理确认对话框");
            ui.checkbox(&mut true, "清理完成后显示结果");
        });
        
        ui.add_space(16.0);
        
        ui.group(|ui| {
            ui.heading("关于");
            ui.label("Resignation Delete v1.0");
            ui.label("开源的离职数据清理工具");
            ui.label("GitHub: https://github.com/ygtec/resignation-delete");
        });
    }
}

fn load_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    
    fonts.font_data.insert(
        "NotoSansCJKsc".to_owned(),
        FontData::from_static(include_bytes!("../assets/NotoSansCJKsc-Regular.otf")).into(),
    );
    
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "NotoSansCJKsc".to_owned());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("NotoSansCJKsc".to_owned());
    
    ctx.set_fonts(fonts);
}

fn setup_custom_style(ctx: &Context) {
    let mut style = Style::default();
    
    style.visuals = Visuals::light();
    
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.window_margin = Margin::same(12.0);
    
    style.visuals.widgets.active.rounding = Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(6.0);
    style.visuals.widgets.open.rounding = Rounding::same(6.0);
    style.visuals.window_rounding = Rounding::same(12.0);
    style.visuals.menu_rounding = Rounding::same(8.0);
    style.visuals.popup_rounding = Rounding::same(8.0);
    style.visuals.window_fill = Color32::from_rgb(250, 250, 252);
    style.visuals.panel_fill = Color32::from_rgb(250, 250, 252);
    style.visuals.extreme_bg_color = Color32::WHITE;
    
    style.visuals.widgets.inactive.weak_bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 220, 230));
    
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(245, 245, 250);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(245, 245, 250);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(180, 180, 200));
    
    style.visuals.widgets.active.weak_bg_fill = Color32::from_rgb(240, 240, 248);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(240, 240, 248);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(150, 150, 180));
    
    style.visuals.selection.bg_fill = Color32::from_rgb(33, 150, 243);
    style.visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    
    style.visuals.hyperlink_color = Color32::from_rgb(33, 150, 243);
    
    ctx.set_style(style);
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
