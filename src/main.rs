use eframe::{egui, App, CreationContext, Frame, NativeOptions, run_native};
use egui::{Color32, Context, FontData, FontDefinitions, FontFamily, RichText, Rounding, Stroke, Style, Vec2, Visuals, Window, Button, ScrollArea, Ui, Layout, Align, Margin};
use std::time::SystemTime;
use chrono::Local;

mod models;
mod cleaner;
mod scanners;

use models::{DataItem, DataType, RiskLevel};
use cleaner::{Cleaner, CleanLog, LogLevel, CleanStatus};
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
    
    pub fn to_data_item(&self) -> DataItem {
        DataItem::new(
            self.id.clone(),
            self.path.clone(),
            DataType::Other,
            self.risk_level.clone(),
            self.size,
            SystemTime::now(),
            SystemTime::now(),
        )
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
}

impl ResignationApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            current_tab: Tab::Cleanup,
            categories: Self::create_default_categories(),
            cleaner: Cleaner::new(),
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
            CategoryItem::new("git-config", "Git Config", "Git 全局配置", "~/.gitconfig", 4_096, RiskLevel::High),
            CategoryItem::new("ssh-keys", "SSH Keys", "SSH 私钥", "~/.ssh/", 16_384, RiskLevel::Critical),
            CategoryItem::new("gnupg", "GNUPG", "GPG 密钥", "~/.gnupg/", 32_768, RiskLevel::High),
            CategoryItem::new("aws-cli", "AWS CLI", "AWS 凭证", "~/.aws/", 8_192, RiskLevel::Critical),
            CategoryItem::new("docker-config", "Docker Config", "Docker 配置", "~/.docker/config.json", 4_096, RiskLevel::High),
            CategoryItem::new("kube-config", "Kubernetes", "K8s 配置", "~/.kube/config", 4_096, RiskLevel::High),
        ];
        Category {
            name: "开发工具".to_string(),
            icon: "\u{1F4BB}".to_string(),
            color: Color32::from_rgb(33, 150, 243),
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
            CategoryItem::new("qq-browser", "QQ浏览器", "QQ浏览器数据", "~/AppData/Local/Tencent/QQBrowser/", 100_000_000, RiskLevel::High),
            CategoryItem::new("360-browser", "360安全浏览器", "360浏览器数据", "~/AppData/Local/360Chrome/", 100_000_000, RiskLevel::High),
            CategoryItem::new("opera", "Opera", "Opera 浏览器数据", "~/AppData/Roaming/Opera Software/", 80_000_000, RiskLevel::High),
            CategoryItem::new("brave", "Brave", "Brave 浏览器数据", "~/AppData/Local/BraveSoftware/", 80_000_000, RiskLevel::High),
        ];
        Category {
            name: "浏览器".to_string(),
            icon: "\u{1F310}".to_string(),
            color: Color32::from_rgb(0, 188, 212),
            items,
            expanded: true,
        }
    }

    fn create_ai_tools_category() -> Category {
        let items = vec![
            CategoryItem::new("cursor", "Cursor", "Cursor AI 编辑器", "~/AppData/Roaming/Cursor/", 50_000_000, RiskLevel::High),
            CategoryItem::new("github-copilot", "GitHub Copilot", "Copilot 扩展", "~/.config/github-copilot/", 10_000_000, RiskLevel::High),
            CategoryItem::new("openai-api", "OpenAI API", "OpenAI API密钥", "~/.openai/", 4_096, RiskLevel::Critical),
            CategoryItem::new("ollama", "Ollama", "Ollama本地模型", "~/.ollama/", 5_000_000_000, RiskLevel::Low),
        ];
        Category {
            name: "AI 工具".to_string(),
            icon: "\u{1F916}".to_string(),
            color: Color32::from_rgb(156, 39, 176),
            items,
            expanded: true,
        }
    }

    fn create_office_category() -> Category {
        let items = vec![
            CategoryItem::new("wechat", "微信", "微信聊天记录", "~/Documents/WeChat Files/", 500_000_000, RiskLevel::Critical),
            CategoryItem::new("qq", "QQ", "QQ聊天记录", "~/Documents/Tencent Files/", 300_000_000, RiskLevel::Critical),
            CategoryItem::new("dingtalk", "钉钉", "钉钉聊天记录", "~/AppData/Roaming/DingTalk/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("feishu", "飞书", "飞书聊天记录", "~/AppData/Roaming/Feishu/", 200_000_000, RiskLevel::Critical),
            CategoryItem::new("wps", "WPS Office", "WPS用户数据", "~/AppData/Local/Kingsoft/", 100_000_000, RiskLevel::High),
            CategoryItem::new("office", "Microsoft Office", "Office缓存", "~/AppData/Local/Microsoft/Office/", 100_000_000, RiskLevel::Medium),
        ];
        Category {
            name: "办公软件".to_string(),
            icon: "\u{1F4CA}".to_string(),
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
            RiskLevel::Critical => "\u4E25\u91CD",
            RiskLevel::High => "\u9AD8\u98CE\u9669",
            RiskLevel::Medium => "\u4E2D\u7B49",
            RiskLevel::Low => "\u4F4E\u98CE\u9669",
        }
    }

    fn get_risk_color(risk: &RiskLevel) -> Color32 {
        match risk {
            RiskLevel::Critical => Color32::from_rgb(244, 67, 54),
            RiskLevel::High => Color32::from_rgb(255, 152, 0),
            RiskLevel::Medium => Color32::from_rgb(255, 193, 7),
            RiskLevel::Low => Color32::from_rgb(76, 175, 80),
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
                        if item.id == scanned_item.id {
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

    fn execute_cleanup(&mut self) {
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
        
        // 清除已清理项的选中状态
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
                ui.heading(RichText::new("Resignation Delete").size(20.0).strong());
                ui.add_space(10.0);
                ui.label(RichText::new("|").color(Color32::GRAY).size(16.0));
                ui.add_space(10.0);
                ui.label(RichText::new("\u79BB\u804C\u6570\u636E\u6E05\u7406\u5DE5\u5177").size(16.0).color(Color32::GRAY));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Cleanup, "\u{1F9F9} \u6E05\u7406");
                ui.selectable_value(&mut self.current_tab, Tab::Logs, "\u{1F4DD} \u65E5\u5FD7");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "\u{2699} \u8BBE\u7F6E");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Cleanup => self.draw_cleanup_panel(ui, ctx),
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
    fn draw_cleanup_panel(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            // 左侧控制面板
            ui.vertical(|ui| {
                ui.set_width(220.0);
                self.draw_control_panel(ui, ctx);
            });
            
            ui.separator();
            
            // 右侧分类列表
            ui.vertical(|ui| {
                self.draw_categories(ui);
            });
        });
    }

    fn draw_control_panel(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.group(|ui| {
            ui.heading("\u64CD\u4F5C\u9762\u677F");
            ui.separator();
            ui.add_space(10.0);
            
            // 扫描按钮
            let scan_btn = Button::new(
                RichText::new("\u{1F50D} \u4E00\u952E\u626B\u63CF").size(15.0)
            )
            .fill(Color32::from_rgb(33, 150, 243))
            .stroke(Stroke::NONE)
            .rounding(8.0)
            .min_size(Vec2::new(180.0, 45.0));
            
            if ui.add_enabled(!self.is_scanning, scan_btn).clicked() {
                self.start_scan();
            }
            
            ui.add_space(10.0);
            
            // 清理按钮
            let has_selected = self.selected_item_count > 0;
            let clean_btn = Button::new(
                RichText::new("\u{1F5D1} \u4E00\u952E\u6E05\u7406").size(15.0)
            )
            .fill(if has_selected { Color32::from_rgb(244, 67, 54) } else { Color32::GRAY })
            .stroke(Stroke::NONE)
            .rounding(8.0)
            .min_size(Vec2::new(180.0, 45.0));
            
            if ui.add_enabled(has_selected, clean_btn).clicked() {
                self.show_confirm_dialog = true;
            }
            
            ui.add_space(20.0);
            
            // 统计信息
            ui.group(|ui| {
                ui.heading("\u7EDF\u8BA1\u4FE1\u606F");
                ui.separator();
                ui.add_space(5.0);
                ui.label(format!("\u603B\u9879\u76EE: {}", self.total_item_count));
                ui.label(format!("\u5DF2\u9009\u62E9: {} \u9879", self.selected_item_count));
                ui.label(format!("\u603B\u5927\u5C0F: {}", Self::format_size(self.total_selected_size)));
                if self.scan_complete {
                    ui.label(RichText::new("\u2713 \u626B\u63CF\u5B8C\u6210").color(Color32::GREEN));
                }
                if self.is_scanning {
                    ui.add(egui::ProgressBar::new(self.scan_progress as f32).text("\u626B\u63CF\u4E2D..."));
                }
            });
            
            ui.add_space(10.0);
            
            // 全选/取消
            ui.horizontal(|ui| {
                if ui.button("\u2611 \u5168\u9009").clicked() {
                    self.select_all(true);
                }
                if ui.button("\u2610 \u53D6\u6D88").clicked() {
                    self.select_all(false);
                }
            });
        });
    }

    fn draw_categories(&mut self, ui: &mut Ui) {
        ui.heading("\u53EF\u6E05\u7406\u9879\u76EE");
        ui.separator();
        
        ScrollArea::vertical().show(ui, |ui| {
            for cat_idx in 0..self.categories.len() {
                let cat = &self.categories[cat_idx];
                let selected = cat.items.iter().filter(|i| i.selected).count();
                let total = cat.items.len();
                
                egui::CollapsingHeader::new(
                    RichText::new(format!("{} {} ({}/{})", cat.icon, cat.name, selected, total))
                        .color(cat.color)
                        .size(16.0)
                        .strong()
                )
                .default_open(cat.expanded)
                .show(ui, |ui| {
                    let all_selected = selected == total;
                    ui.horizontal(|ui| {
                        let mut check = all_selected;
                        if ui.checkbox(&mut check, RichText::new("\u6B64\u5206\u7C7B\u5168\u9009").color(cat.color)).changed() {
                            self.select_all_in_category(cat_idx, check);
                        }
                    });
                    
                    ui.separator();
                    ui.add_space(5.0);
                    
                    for item_idx in 0..cat.items.len() {
                        let item = &cat.items[item_idx];
                        
                        let bg_color = if item.selected {
                            Color32::from_rgb(232, 245, 255)
                        } else if item_idx % 2 == 0 {
                            Color32::from_rgb(250, 250, 250)
                        } else {
                            Color32::from_rgb(255, 255, 255)
                        };
                        
                        egui::Frame::none()
                            .fill(bg_color)
                            .rounding(6.0)
                            .inner_margin(Margin::same(8.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let mut checked = item.selected;
                                    if ui.checkbox(&mut checked, "").changed() {
                                        self.categories[cat_idx].items[item_idx].selected = checked;
                                        self.update_counts();
                                    }
                                    
                                    ui.vertical(|ui| {
                                        // 名称和描述
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&item.name).strong().size(14.0));
                                            if !item.description.is_empty() {
                                                ui.label(RichText::new(&item.description).color(Color32::GRAY).size(12.0));
                                            }
                                        });
                                        
                                        // 路径、大小、风险
                                        ui.horizontal(|ui| {
                                            let risk_color = Self::get_risk_color(&item.risk_level);
                                            let risk_label = Self::get_risk_label(&item.risk_level);
                                            ui.label(RichText::new(format!("[{}]", risk_label)).color(risk_color).size(11.0).strong());
                                            
                                            ui.label(RichText::new(&item.path).color(Color32::GRAY).size(11.0));
                                            
                                            if item.size > 0 {
                                                ui.label(RichText::new(Self::format_size(item.size)).color(Color32::GRAY).size(11.0));
                                            }
                                            
                                            if item.detected {
                                                ui.label(RichText::new("\u2713 \u5DF2\u53D1\u73B0").color(Color32::GREEN).size(11.0).strong());
                                            }
                                        });
                                    });
                                });
                            });
                        
                        ui.add_space(4.0);
                    }
                });
                
                ui.add_space(8.0);
            }
        });
    }

    fn draw_confirm_dialog(&mut self, ctx: &Context) {
        Window::new("\u786E\u8BA4\u6E05\u7406")
            .collapsible(false)
            .resizable(true)
            .default_width(500.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("\u786E\u8BA4\u8981\u6E05\u7406\u4EE5\u4E0B\u6570\u636E\u5417\uFF1F").size(16.0).strong().color(Color32::RED));
                ui.add_space(8.0);
                ui.label(format!("\u5171 {} \u4E2A\u9879\u76EE\uFF0C\u603B\u5927\u5C0F {}", self.selected_item_count, Self::format_size(self.total_selected_size)));
                ui.add_space(8.0);
                
                ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    for cat in &self.categories {
                        for item in &cat.items {
                            if item.selected {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("{} {}", cat.icon, item.name)).strong().size(13.0));
                                    ui.label(RichText::new(&item.path).color(Color32::GRAY).size(12.0));
                                });
                            }
                        }
                    }
                });
                
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button("\u786E\u8BA4\u6E05\u7406").clicked() {
                        self.show_confirm_dialog = false;
                        self.execute_cleanup();
                    }
                    if ui.button("\u53D6\u6D88").clicked() {
                        self.show_confirm_dialog = false;
                    }
                });
            });
    }

    fn draw_success_dialog(&mut self, ctx: &Context) {
        Window::new("\u6E05\u7406\u7ED3\u679C")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("\u2713 \u6E05\u7406\u5B8C\u6210\uFF01").size(18.0).strong().color(Color32::GREEN));
                ui.add_space(12.0);
                ui.label(format!("\u6210\u529F\u6E05\u7406: {} \u4E2A\u9879\u76EE", self.success_count));
                ui.label(format!("\u6E05\u7406\u5931\u8D25: {} \u4E2A\u9879\u76EE", self.fail_count));
                ui.add_space(16.0);
                if ui.button("\u786E\u5B9A").clicked() {
                    self.show_success_dialog = false;
                }
            });
    }

    fn draw_logs_panel(&mut self, ui: &mut Ui) {
        ui.heading("\u64CD\u4F5C\u65E5\u5FD7");
        ui.separator();
        
        ScrollArea::vertical().show(ui, |ui| {
            let logs = self.cleaner.logs();
            if logs.is_empty() {
                ui.label(RichText::new("\u6682\u65E0\u65E5\u5FD7\u8BB0\u5F55").color(Color32::GRAY).size(14.0));
            } else {
                for log in logs {
                    let color = match log.level {
                        LogLevel::Info => Color32::GREEN,
                        LogLevel::Warning => Color32::YELLOW,
                        LogLevel::Error => Color32::RED,
                    };
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("[{}]", log.timestamp)).color(Color32::GRAY).size(12.0));
                        let level_str = match log.level {
                            LogLevel::Info => "INFO",
                            LogLevel::Warning => "WARN",
                            LogLevel::Error => "ERROR",
                        };
                        ui.label(RichText::new(format!("[{}]", level_str)).color(color).size(12.0).strong());
                        ui.label(RichText::new(&log.message).size(12.0));
                    });
                }
            }
        });
    }

    fn draw_settings_panel(&mut self, ui: &mut Ui) {
        ui.heading("\u8BBE\u7F6E");
        ui.separator();
        
        ui.group(|ui| {
            ui.heading("\u6E05\u7406\u9009\u9879");
            ui.checkbox(&mut true, "\u6E05\u7406\u540E\u8986\u5199\u6587\u4EF6\uFF08\u5B89\u5168\u5220\u9664\uFF09");
            ui.checkbox(&mut true, "\u663E\u793A\u6E05\u7406\u786E\u8BA4\u5BF9\u8BDD\u6846");
            ui.checkbox(&mut true, "\u6E05\u7406\u5B8C\u6210\u540E\u663E\u793A\u7ED3\u679C");
        });
        
        ui.add_space(16.0);
        
        ui.group(|ui| {
            ui.heading("\u5173\u4E8E");
            ui.label("Resignation Delete v1.0");
            ui.label("\u5F00\u6E90\u7684\u79BB\u804C\u6570\u636E\u6E05\u7406\u5DE5\u5177");
            ui.label("GitHub: https://github.com/ygtec/resignation-delete");
        });
    }
}

fn load_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    
    // 加载嵌入式中文字体
    fonts.font_data.insert(
        "NotoSansCJKsc".to_owned(),
        FontData::from_static(include_bytes!("../assets/NotoSansCJKsc-Regular.otf")).into(),
    );
    
    // 将中文字体设置为优先字体
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "NotoSansCJKsc".to_owned());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("NotoSansCJKsc".to_owned());
    
    ctx.set_fonts(fonts);
}

fn setup_custom_style(ctx: &Context) {
    let mut style = Style::default();
    
    // 使用浅色主题
    style.visuals = Visuals::light();
    
    // 间距设置
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(12.0, 8.0);
    style.spacing.window_margin = Margin::same(12.0);
    style.spacing.window_padding = Vec2::new(12.0, 12.0);
    
    // 圆角设置
    style.visuals.widgets.active.rounding = Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(6.0);
    style.visuals.widgets.open.rounding = Rounding::same(6.0);
    style.visuals.window_rounding = Rounding::same(10.0);
    style.visuals.menu_rounding = Rounding::same(8.0);
    style.visuals.popup_rounding = Rounding::same(8.0);
    
    // 背景颜色
    style.visuals.window_fill = Color32::from_rgb(250, 250, 252);
    style.visuals.panel_fill = Color32::from_rgb(250, 250, 252);
    style.visuals.extreme_bg_color = Color32::WHITE;
    
    // 按钮和控件样式
    style.visuals.widgets.inactive.weak_bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 220, 230));
    
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(245, 245, 250);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(245, 245, 250);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(180, 180, 200));
    
    style.visuals.widgets.active.weak_bg_fill = Color32::from_rgb(240, 240, 248);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(240, 240, 248);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(150, 150, 180));
    
    // 选中样式
    style.visuals.selection.bg_fill = Color32::from_rgb(33, 150, 243);
    style.visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    
    // 链接颜色
    style.visuals.hyperlink_color = Color32::from_rgb(33, 150, 243);
    
    ctx.set_style(style);
}

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Resignation Delete - \u79BB\u804C\u6570\u636E\u6E05\u7406\u5DE5\u5177"),
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
