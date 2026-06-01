mod models;
mod scanners;
mod cleaner;

use scanners::{Scanner, git_ssh::GitSshScanner, browsers::BrowsersScanner, jetbrains::JetBrainsScanner, vscode::VSCodeScanner, ai_tools::AIToolsScanner};
use cleaner::{Cleaner, CleanLog, LogLevel};
use models::{DataItem, RiskLevel};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    simple_logger::init_with_level(log::Level::Info).ok();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Resignation Delete - 离职数据清理工具",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::<ResignationDeleteApp>::default())
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/NotoSansCJKsc-Regular.otf")).into(),
    );
    
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    
    ctx.set_fonts(fonts);
}

#[derive(Clone)]
struct Category {
    name: String,
    icon: String,
    items: Vec<CategoryItem>,
    expanded: bool,
    selected_count: usize,
    total_count: usize,
}

#[derive(Clone)]
struct CategoryItem {
    id: String,
    name: String,
    description: String,
    path: String,
    size: u64,
    risk_level: RiskLevel,
    selected: bool,
    scanned: bool,
}

struct ResignationDeleteApp {
    categories: Vec<Category>,
    show_confirm_dialog: bool,
    show_result_dialog: bool,
    cleaner: Cleaner,
    logs: Vec<CleanLog>,
    is_scanning: bool,
    is_cleaning: bool,
    scan_progress: f32,
    total_items: usize,
    selected_items: usize,
    cleaned_count: usize,
    failed_count: usize,
}

impl Default for ResignationDeleteApp {
    fn default() -> Self {
        let categories = vec![
            Category {
                name: "开发工具".to_string(),
                icon: "code".to_string(),
                items: vec![
                    CategoryItem {
                        id: "git_ssh".to_string(),
                        name: "Git SSH 密钥".to_string(),
                        description: "Git SSH 私钥和公钥文件".to_string(),
                        path: "~/.ssh/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "git_config".to_string(),
                        name: "Git 全局配置".to_string(),
                        description: "Git 用户名、邮箱和全局配置".to_string(),
                        path: "~/.gitconfig".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "jetbrains".to_string(),
                        name: "JetBrains IDE".to_string(),
                        description: "IntelliJ IDEA, PyCharm, WebStorm 等账号配置".to_string(),
                        path: "~/.config/JetBrains/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "vscode".to_string(),
                        name: "VSCode".to_string(),
                        description: "Visual Studio Code 用户配置和登录信息".to_string(),
                        path: "~/.vscode/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "npm".to_string(),
                        name: "NPM 配置".to_string(),
                        description: "NPM 登录令牌和配置文件".to_string(),
                        path: "~/.npmrc".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "docker".to_string(),
                        name: "Docker 配置".to_string(),
                        description: "Docker 登录信息和配置文件".to_string(),
                        path: "~/.docker/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "ssh".to_string(),
                        name: "SSH 配置".to_string(),
                        description: "SSH 客户端配置文件和 known_hosts".to_string(),
                        path: "~/.ssh/config".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                ],
                expanded: true,
                selected_count: 7,
                total_count: 7,
            },
            Category {
                name: "浏览器".to_string(),
                icon: "browser".to_string(),
                items: vec![
                    CategoryItem {
                        id: "chrome".to_string(),
                        name: "Chrome".to_string(),
                        description: "Chrome 浏览器历史记录、Cookie、密码".to_string(),
                        path: "~/AppData/Local/Google/Chrome/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "edge".to_string(),
                        name: "Edge".to_string(),
                        description: "Edge 浏览器历史记录、Cookie、密码".to_string(),
                        path: "~/AppData/Local/Microsoft/Edge/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "firefox".to_string(),
                        name: "Firefox".to_string(),
                        description: "Firefox 浏览器历史记录、Cookie、密码".to_string(),
                        path: "~/AppData/Roaming/Mozilla/Firefox/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                ],
                expanded: true,
                selected_count: 3,
                total_count: 3,
            },
            Category {
                name: "AI 工具".to_string(),
                icon: "ai".to_string(),
                items: vec![
                    CategoryItem {
                        id: "cursor".to_string(),
                        name: "Cursor".to_string(),
                        description: "Cursor AI 编辑器账号和配置".to_string(),
                        path: "~/.cursor/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "claude".to_string(),
                        name: "Claude Desktop".to_string(),
                        description: "Claude 桌面应用账号信息".to_string(),
                        path: "~/AppData/Roaming/Claude/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "github_copilot".to_string(),
                        name: "GitHub Copilot".to_string(),
                        description: "GitHub Copilot 授权和配置".to_string(),
                        path: "~/.config/github-copilot/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "kimi".to_string(),
                        name: "Kimi".to_string(),
                        description: "Kimi AI 助手账号信息".to_string(),
                        path: "~/.kimi/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Low,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "qwen".to_string(),
                        name: "通义千问".to_string(),
                        description: "通义千问 AI 助手账号信息".to_string(),
                        path: "~/.qwen/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Low,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "opencode".to_string(),
                        name: "OpenCode".to_string(),
                        description: "OpenCode AI 编程助手账号和配置".to_string(),
                        path: "~/.opencode/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "trae".to_string(),
                        name: "Trae".to_string(),
                        description: "Trae AI 编辑器账号和配置".to_string(),
                        path: "~/.trae/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                ],
                expanded: true,
                selected_count: 7,
                total_count: 7,
            },
            Category {
                name: "办公软件".to_string(),
                icon: "office".to_string(),
                items: vec![
                    CategoryItem {
                        id: "wps".to_string(),
                        name: "WPS Office".to_string(),
                        description: "WPS Office 登录账号和文档记录".to_string(),
                        path: "~/AppData/Local/Kingsoft/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "wechat".to_string(),
                        name: "微信".to_string(),
                        description: "微信聊天记录和登录信息".to_string(),
                        path: "~/Documents/WeChat Files/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "qq".to_string(),
                        name: "QQ".to_string(),
                        description: "QQ 聊天记录和登录信息".to_string(),
                        path: "~/Documents/Tencent Files/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "dingtalk".to_string(),
                        name: "钉钉".to_string(),
                        description: "钉钉登录信息和聊天记录".to_string(),
                        path: "~/AppData/Roaming/DingTalk/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "feishu".to_string(),
                        name: "飞书".to_string(),
                        description: "飞书登录信息和聊天记录".to_string(),
                        path: "~/AppData/Roaming/Feishu/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "microsoft_office".to_string(),
                        name: "Microsoft Office".to_string(),
                        description: "Office 登录账号和文档缓存".to_string(),
                        path: "~/AppData/Local/Microsoft/Office/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "企业微信".to_string(),
                        name: "企业微信".to_string(),
                        description: "企业微信聊天记录和登录信息".to_string(),
                        path: "~/Documents/WXWork/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                ],
                expanded: true,
                selected_count: 7,
                total_count: 7,
            },
        ];
        
        let total_items = categories.iter().map(|c| c.items.len()).sum();
        let selected_items = categories.iter().map(|c| c.selected_count).sum();
        
        Self {
            categories,
            show_confirm_dialog: false,
            show_result_dialog: false,
            cleaner: Cleaner::new(),
            logs: Vec::new(),
            is_scanning: false,
            is_cleaning: false,
            scan_progress: 0.0,
            total_items,
            selected_items,
            cleaned_count: 0,
            failed_count: 0,
        }
    }
}

impl eframe::App for ResignationDeleteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部标题栏
        egui::TopBottomPanel::top("header").exact_height(60.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("Resignation Delete").size(28.0).strong());
                ui.label(egui::RichText::new("离职数据清理工具").size(16.0).color(egui::Color32::GRAY));
            });
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar").exact_height(40.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                if self.is_scanning {
                    ui.add(egui::ProgressBar::new(self.scan_progress).show_percentage().desired_width(200.0));
                    ui.label("正在扫描系统...");
                } else if self.is_cleaning {
                    ui.label("正在清理数据... 请稍候");
                } else {
                    ui.label(format!(
                        "共 {} 个项目 | 已选择 {} 个 | 已清理 {} 个",
                        self.total_items, self.selected_items, self.cleaned_count
                    ));
                }
            });
        });

        // 主内容区
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 左侧控制面板
                ui.vertical(|ui| {
                    ui.set_width(250.0);
                    
                    // 一键扫描按钮
                    ui.add_space(20.0);
                    let scan_btn = ui.add_sized(
                        [230.0, 50.0],
                        egui::Button::new(
                            egui::RichText::new("一键扫描").size(18.0).strong()
                        ).fill(egui::Color32::from_rgb(0, 120, 212))
                    );
                    if scan_btn.clicked() && !self.is_scanning && !self.is_cleaning {
                        self.perform_scan();
                    }
                    
                    ui.add_space(15.0);
                    
                    // 一键清理按钮
                    let clean_btn = ui.add_sized(
                        [230.0, 50.0],
                        egui::Button::new(
                            egui::RichText::new("一键清理").size(18.0).strong()
                        ).fill(if self.selected_items > 0 {
                            egui::Color32::from_rgb(220, 53, 69)
                        } else {
                            egui::Color32::GRAY
                        })
                    );
                    if clean_btn.clicked() && self.selected_items > 0 && !self.is_cleaning {
                        self.show_confirm_dialog = true;
                    }
                    
                    ui.add_space(30.0);
                    
                    // 统计信息
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(248, 249, 250))
                        .show(ui, |ui| {
                            ui.set_width(230.0);
                            ui.heading("扫描统计");
                            ui.separator();
                            
                            ui.horizontal(|ui| {
                                ui.label("总项目:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(self.total_items.to_string());
                                });
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("已选择:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.colored_label(egui::Color32::from_rgb(0, 120, 212), self.selected_items.to_string());
                                });
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("已清理:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.colored_label(egui::Color32::from_rgb(40, 167, 69), self.cleaned_count.to_string());
                                });
                            });
                            
                            if self.failed_count > 0 {
                                ui.horizontal(|ui| {
                                    ui.label("清理失败:");
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.colored_label(egui::Color32::RED, self.failed_count.to_string());
                                    });
                                });
                            }
                        });
                    
                    ui.add_space(20.0);
                    
                    // 全选/取消按钮
                    ui.horizontal(|ui| {
                        if ui.button("全选").clicked() {
                            self.select_all(true);
                        }
                        if ui.button("取消全选").clicked() {
                            self.select_all(false);
                        }
                    });
                });
                
                ui.separator();
                
                // 右侧项目列表
                ui.vertical(|ui| {
                    self.draw_categories(ui);
                });
            });
        });

        // 确认对话框
        if self.show_confirm_dialog {
            self.draw_confirm_dialog(ctx);
        }
        
        // 结果对话框
        if self.show_result_dialog {
            self.draw_result_dialog(ctx);
        }
    }
}

impl ResignationDeleteApp {
    fn select_all(&mut self, select: bool) {
        for category in &mut self.categories {
            for item in &mut category.items {
                item.selected = select;
            }
            category.selected_count = if select { category.items.len() } else { 0 };
        }
        self.selected_items = if select { self.total_items } else { 0 };
    }

    fn perform_scan(&mut self) {
        self.is_scanning = true;
        self.scan_progress = 0.0;
        
        let scanners: Vec<Box<dyn Scanner>> = vec![
            Box::new(GitSshScanner),
            Box::new(BrowsersScanner),
            Box::new(JetBrainsScanner),
            Box::new(VSCodeScanner),
            Box::new(AIToolsScanner),
        ];
        
        let scanner_count = scanners.len() as f32;
        for (i, scanner) in scanners.iter().enumerate() {
            self.scan_progress = (i as f32) / scanner_count;
            let scanned = scanner.scan();
            
            // 更新对应分类的项目状态
            for data_item in scanned {
                for category in &mut self.categories {
                    for item in &mut category.items {
                        if item.id == data_item.id || data_item.path.contains(&item.id) {
                            item.scanned = true;
                            item.size = data_item.size;
                            item.path = data_item.path.clone();
                            if let Some(desc) = &data_item.description {
                                item.description = desc.clone();
                            }
                        }
                    }
                }
            }
        }
        
        self.scan_progress = 1.0;
        self.is_scanning = false;
        
        self.logs.push(CleanLog {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: LogLevel::Info,
            message: "扫描完成".to_string(),
            item_path: None,
        });
    }

    fn draw_categories(&mut self, ui: &mut egui::Ui) {
        ui.heading("可清理项目");
        ui.separator();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for cat_idx in 0..self.categories.len() {
                let category = &mut self.categories[cat_idx];
                
                // 分类标题栏
                let header_response = egui::CollapsingHeader::new(
                    egui::RichText::new(format!("{} ({}/{})", 
                        category.name, 
                        category.selected_count, 
                        category.total_count
                    )).size(16.0).strong()
                )
                .default_open(category.expanded)
                .show(ui, |ui| {
                    // 分类全选按钮
                    ui.horizontal(|ui| {
                        let all_selected = category.selected_count == category.total_count;
                        let mut select_all = all_selected;
                        if ui.checkbox(&mut select_all, "全选此分类").changed() {
                            for item in &mut category.items {
                                item.selected = select_all;
                            }
                            category.selected_count = if select_all { category.total_count } else { 0 };
                            self.update_selected_count();
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // 项目列表
                    for item_idx in 0..category.items.len() {
                        let item = &mut category.items[item_idx];
                        
                        let risk_color = match item.risk_level {
                            RiskLevel::Critical => egui::Color32::RED,
                            RiskLevel::High => egui::Color32::from_rgb(255, 140, 0),
                            RiskLevel::Medium => egui::Color32::from_rgb(255, 193, 7),
                            RiskLevel::Low => egui::Color32::from_rgb(40, 167, 69),
                        };
                        
                        let risk_text = match item.risk_level {
                            RiskLevel::Critical => "严重",
                            RiskLevel::High => "高风险",
                            RiskLevel::Medium => "中等",
                            RiskLevel::Low => "低风险",
                        };
                        
                        let bg_color = if item.selected {
                            egui::Color32::from_rgb(230, 245, 255)
                        } else {
                            egui::Color32::WHITE
                        };
                        
                        egui::Frame::group(ui.style())
                            .fill(bg_color)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let mut checked = item.selected;
                                    if ui.checkbox(&mut checked, "").changed() {
                                        item.selected = checked;
                                        category.selected_count = category.items.iter().filter(|i| i.selected).count();
                                        self.update_selected_count();
                                    }
                                    
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(&item.name).strong());
                                            ui.colored_label(risk_color, format!("[{}]", risk_text));
                                            if item.scanned {
                                                ui.colored_label(egui::Color32::GREEN, "已发现");
                                            }
                                        });
                                        
                                        ui.label(egui::RichText::new(&item.description).size(12.0).color(egui::Color32::DARK_GRAY));
                                        
                                        if item.scanned && item.size > 0 {
                                            ui.label(egui::RichText::new(format!("路径: {} | 大小: {}", 
                                                item.path, Self::format_size(item.size)))
                                                .size(11.0).color(egui::Color32::GRAY));
                                        } else {
                                            ui.label(egui::RichText::new(format!("路径: {}", item.path))
                                                .size(11.0).color(egui::Color32::GRAY));
                                        }
                                    });
                                });
                            });
                        
                        ui.add_space(3.0);
                    }
                });
                
                category.expanded = header_response.fully_open();
                ui.add_space(10.0);
            }
        });
    }

    fn update_selected_count(&mut self) {
        self.selected_items = self.categories.iter()
            .map(|c| c.selected_count)
            .sum();
    }

    fn draw_confirm_dialog(&mut self, ctx: &egui::Context) {
        let mut should_close = false;
        
        egui::Window::new("确认清理")
            .collapsible(false)
            .resizable(false)
            .fixed_size([450.0, 250.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(15.0);
                    ui.colored_label(
                        egui::Color32::RED,
                        egui::RichText::new("⚠ 警告").size(24.0).strong()
                    );
                    ui.add_space(15.0);
                    ui.label(egui::RichText::new("即将永久删除以下数据:").size(16.0));
                    ui.add_space(10.0);
                    
                    // 显示要删除的项目列表
                    let mut delete_list = Vec::new();
                    for category in &self.categories {
                        for item in &category.items {
                            if item.selected {
                                delete_list.push(format!("{} - {}", category.name, item.name));
                            }
                        }
                    }
                    
                    for item_name in &delete_list {
                        ui.label(egui::RichText::new(format!("• {}", item_name)).size(14.0));
                    }
                    
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(format!("共 {} 个项目", self.selected_items)).size(14.0).strong());
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("删除后无法恢复！").size(14.0).color(egui::Color32::RED));
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        let confirm_btn = ui.add_sized(
                            [130.0, 40.0],
                            egui::Button::new(
                                egui::RichText::new("确认删除").size(16.0).strong()
                            ).fill(egui::Color32::from_rgb(220, 53, 69))
                        );
                        if confirm_btn.clicked() {
                            self.execute_cleanup();
                            should_close = true;
                        }
                        
                        ui.add_space(30.0);
                        
                        if ui.add_sized([130.0, 40.0], egui::Button::new(
                            egui::RichText::new("取消").size(16.0)
                        )).clicked() {
                            should_close = true;
                        }
                    });
                });
            });
        
        if should_close {
            self.show_confirm_dialog = false;
        }
    }

    fn draw_result_dialog(&mut self, ctx: &egui::Context) {
        let mut should_close = false;
        
        egui::Window::new("清理完成")
            .collapsible(false)
            .resizable(false)
            .fixed_size([400.0, 200.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    if self.failed_count == 0 {
                        ui.colored_label(
                            egui::Color32::GREEN,
                            egui::RichText::new("清理成功！").size(24.0).strong()
                        );
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 140, 0),
                            egui::RichText::new("清理完成（部分失败）").size(24.0).strong()
                        );
                    }
                    
                    ui.add_space(20.0);
                    ui.label(format!("成功清理: {} 个项目", self.cleaned_count));
                    if self.failed_count > 0 {
                        ui.colored_label(egui::Color32::RED, format!("清理失败: {} 个项目", self.failed_count));
                    }
                    ui.add_space(30.0);
                    
                    if ui.add_sized([120.0, 40.0], egui::Button::new(
                        egui::RichText::new("确定").size(16.0)
                    )).clicked() {
                        should_close = true;
                    }
                });
            });
        
        if should_close {
            self.show_result_dialog = false;
        }
    }

    fn execute_cleanup(&mut self) {
        self.is_cleaning = true;
        self.cleaned_count = 0;
        self.failed_count = 0;
        
        // 收集选中的项目
        let mut items_to_clean: Vec<DataItem> = Vec::new();
        for category in &self.categories {
            for item in &category.items {
                if item.selected && item.scanned {
                    items_to_clean.push(DataItem {
                        id: item.id.clone(),
                        path: item.path.clone(),
                        data_type: models::DataType::Document,
                        risk_level: item.risk_level,
                        size: item.size,
                        created_at: None,
                        modified_at: None,
                        description: Some(item.description.clone()),
                    });
                }
            }
        }
        
        self.cleaner.clear_tasks();
        self.cleaner.add_tasks(items_to_clean.clone());
        
        let result = self.cleaner.clean_all(|_| true);
        
        match result {
            Ok(_) => {
                self.cleaned_count = items_to_clean.len();
                
                // 标记已清理的项目
                for category in &mut self.categories {
                    for item in &mut category.items {
                        if item.selected {
                            item.scanned = false;
                            item.selected = false;
                        }
                    }
                    category.selected_count = 0;
                }
                self.selected_items = 0;
                
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Info,
                    message: format!("成功清理 {} 个项目", self.cleaned_count),
                    item_path: None,
                });
            }
            Err(e) => {
                self.failed_count = items_to_clean.len();
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Error,
                    message: format!("清理失败: {}", e),
                    item_path: None,
                });
            }
        }
        
        self.is_cleaning = false;
        self.show_result_dialog = true;
    }

    fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        
        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }
}
