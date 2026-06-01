mod models;
mod scanners;
mod cleaner;

use scanners::{Scanner, git_ssh::GitSshScanner, browsers::BrowsersScanner, jetbrains::JetBrainsScanner, vscode::VSCodeScanner, ai_tools::AIToolsScanner};
use cleaner::{Cleaner, CleanLog, LogLevel};
use models::{DataItem, RiskLevel};
use eframe::egui;
use std::collections::HashSet;

fn main() -> Result<(), eframe::Error> {
    simple_logger::init_with_level(log::Level::Info).ok();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Resignation Delete - 绂昏亴鏁版嵁娓呯悊宸ュ叿",
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
                name: "寮€鍙戝伐鍏?.to_string(),
                icon: "code".to_string(),
                items: vec![
                    CategoryItem {
                        id: "git_ssh".to_string(),
                        name: "Git SSH 瀵嗛挜".to_string(),
                        description: "Git SSH 绉侀挜鍜屽叕閽ユ枃浠?.to_string(),
                        path: "~/.ssh/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "git_config".to_string(),
                        name: "Git 鍏ㄥ眬閰嶇疆".to_string(),
                        description: "Git 鐢ㄦ埛鍚嶃€侀偖绠卞拰鍏ㄥ眬閰嶇疆".to_string(),
                        path: "~/.gitconfig".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "jetbrains".to_string(),
                        name: "JetBrains IDE".to_string(),
                        description: "IntelliJ IDEA, PyCharm, WebStorm 绛夎处鍙烽厤缃?.to_string(),
                        path: "~/.config/JetBrains/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "vscode".to_string(),
                        name: "VSCode".to_string(),
                        description: "Visual Studio Code 鐢ㄦ埛閰嶇疆鍜岀櫥褰曚俊鎭?.to_string(),
                        path: "~/.vscode/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "npm".to_string(),
                        name: "NPM 閰嶇疆".to_string(),
                        description: "NPM 鐧诲綍浠ょ墝鍜岄厤缃枃浠?.to_string(),
                        path: "~/.npmrc".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "docker".to_string(),
                        name: "Docker 閰嶇疆".to_string(),
                        description: "Docker 鐧诲綍淇℃伅鍜岄厤缃枃浠?.to_string(),
                        path: "~/.docker/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "ssh".to_string(),
                        name: "SSH 閰嶇疆".to_string(),
                        description: "SSH 瀹㈡埛绔厤缃枃浠跺拰 known_hosts".to_string(),
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
                name: "娴忚鍣?.to_string(),
                icon: "browser".to_string(),
                items: vec![
                    CategoryItem {
                        id: "chrome".to_string(),
                        name: "Chrome".to_string(),
                        description: "Chrome 娴忚鍣ㄥ巻鍙茶褰曘€丆ookie銆佸瘑鐮?.to_string(),
                        path: "~/AppData/Local/Google/Chrome/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "edge".to_string(),
                        name: "Edge".to_string(),
                        description: "Edge 娴忚鍣ㄥ巻鍙茶褰曘€丆ookie銆佸瘑鐮?.to_string(),
                        path: "~/AppData/Local/Microsoft/Edge/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "firefox".to_string(),
                        name: "Firefox".to_string(),
                        description: "Firefox 娴忚鍣ㄥ巻鍙茶褰曘€丆ookie銆佸瘑鐮?.to_string(),
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
                name: "AI 宸ュ叿".to_string(),
                icon: "ai".to_string(),
                items: vec![
                    CategoryItem {
                        id: "cursor".to_string(),
                        name: "Cursor".to_string(),
                        description: "Cursor AI 缂栬緫鍣ㄨ处鍙峰拰閰嶇疆".to_string(),
                        path: "~/.cursor/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "claude".to_string(),
                        name: "Claude Desktop".to_string(),
                        description: "Claude 妗岄潰搴旂敤璐﹀彿淇℃伅".to_string(),
                        path: "~/AppData/Roaming/Claude/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "github_copilot".to_string(),
                        name: "GitHub Copilot".to_string(),
                        description: "GitHub Copilot 鎺堟潈鍜岄厤缃?.to_string(),
                        path: "~/.config/github-copilot/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "kimi".to_string(),
                        name: "Kimi".to_string(),
                        description: "Kimi AI 鍔╂墜璐﹀彿淇℃伅".to_string(),
                        path: "~/.kimi/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Low,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "qwen".to_string(),
                        name: "閫氫箟鍗冮棶".to_string(),
                        description: "閫氫箟鍗冮棶 AI 鍔╂墜璐﹀彿淇℃伅".to_string(),
                        path: "~/.qwen/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Low,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "opencode".to_string(),
                        name: "OpenCode".to_string(),
                        description: "OpenCode AI 缂栫▼鍔╂墜璐﹀彿鍜岄厤缃?.to_string(),
                        path: "~/.opencode/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "trae".to_string(),
                        name: "Trae".to_string(),
                        description: "Trae AI 缂栬緫鍣ㄨ处鍙峰拰閰嶇疆".to_string(),
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
                name: "鍔炲叕杞欢".to_string(),
                icon: "office".to_string(),
                items: vec![
                    CategoryItem {
                        id: "wps".to_string(),
                        name: "WPS Office".to_string(),
                        description: "WPS Office 鐧诲綍璐﹀彿鍜屾枃妗ｈ褰?.to_string(),
                        path: "~/AppData/Local/Kingsoft/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "wechat".to_string(),
                        name: "寰俊".to_string(),
                        description: "寰俊鑱婂ぉ璁板綍鍜岀櫥褰曚俊鎭?.to_string(),
                        path: "~/Documents/WeChat Files/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "qq".to_string(),
                        name: "QQ".to_string(),
                        description: "QQ 鑱婂ぉ璁板綍鍜岀櫥褰曚俊鎭?.to_string(),
                        path: "~/Documents/Tencent Files/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::High,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "dingtalk".to_string(),
                        name: "閽夐拤".to_string(),
                        description: "閽夐拤鐧诲綍淇℃伅鍜岃亰澶╄褰?.to_string(),
                        path: "~/AppData/Roaming/DingTalk/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "feishu".to_string(),
                        name: "椋炰功".to_string(),
                        description: "椋炰功鐧诲綍淇℃伅鍜岃亰澶╄褰?.to_string(),
                        path: "~/AppData/Roaming/Feishu/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "microsoft_office".to_string(),
                        name: "Microsoft Office".to_string(),
                        description: "Office 鐧诲綍璐﹀彿鍜屾枃妗ｇ紦瀛?.to_string(),
                        path: "~/AppData/Local/Microsoft/Office/".to_string(),
                        size: 0,
                        risk_level: RiskLevel::Medium,
                        selected: true,
                        scanned: false,
                    },
                    CategoryItem {
                        id: "浼佷笟寰俊".to_string(),
                        name: "浼佷笟寰俊".to_string(),
                        description: "浼佷笟寰俊鑱婂ぉ璁板綍鍜岀櫥褰曚俊鎭?.to_string(),
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
        // 椤堕儴鏍囬鏍?        egui::TopBottomPanel::top("header").exact_height(60.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("Resignation Delete").size(28.0).strong());
                ui.label(egui::RichText::new("绂昏亴鏁版嵁娓呯悊宸ュ叿").size(16.0).color(egui::Color32::GRAY));
            });
        });

        // 搴曢儴鐘舵€佹爮
        egui::TopBottomPanel::bottom("status_bar").exact_height(40.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                if self.is_scanning {
                    ui.add(egui::ProgressBar::new(self.scan_progress).show_percentage().desired_width(200.0));
                    ui.label("姝ｅ湪鎵弿绯荤粺...");
                } else if self.is_cleaning {
                    ui.label("姝ｅ湪娓呯悊鏁版嵁... 璇风◢鍊?);
                } else {
                    ui.label(format!(
                        "鍏?{} 涓」鐩?| 宸查€夋嫨 {} 涓?| 宸叉竻鐞?{} 涓?,
                        self.total_items, self.selected_items, self.cleaned_count
                    ));
                }
            });
        });

        // 涓诲唴瀹瑰尯
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 宸︿晶鎺у埗闈㈡澘
                ui.vertical(|ui| {
                    ui.set_width(250.0);
                    
                    // 涓€閿壂鎻忔寜閽?                    ui.add_space(20.0);
                    let scan_btn = ui.add_sized(
                        [230.0, 50.0],
                        egui::Button::new(
                            egui::RichText::new("涓€閿壂鎻?).size(18.0).strong()
                        ).fill(egui::Color32::from_rgb(0, 120, 212))
                    );
                    if scan_btn.clicked() && !self.is_scanning && !self.is_cleaning {
                        self.perform_scan();
                    }
                    
                    ui.add_space(15.0);
                    
                    // 涓€閿竻鐞嗘寜閽?                    let clean_btn = ui.add_sized(
                        [230.0, 50.0],
                        egui::Button::new(
                            egui::RichText::new("涓€閿竻鐞?).size(18.0).strong()
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
                    
                    // 缁熻淇℃伅
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(248, 249, 250))
                        .show(ui, |ui| {
                            ui.set_width(230.0);
                            ui.heading("鎵弿缁熻");
                            ui.separator();
                            
                            ui.horizontal(|ui| {
                                ui.label("鎬婚」鐩?");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(self.total_items.to_string());
                                });
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("宸查€夋嫨:");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.colored_label(egui::Color32::from_rgb(0, 120, 212), self.selected_items.to_string());
                                });
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("宸叉竻鐞?");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.colored_label(egui::Color32::from_rgb(40, 167, 69), self.cleaned_count.to_string());
                                });
                            });
                            
                            if self.failed_count > 0 {
                                ui.horizontal(|ui| {
                                    ui.label("娓呯悊澶辫触:");
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.colored_label(egui::Color32::RED, self.failed_count.to_string());
                                    });
                                });
                            }
                        });
                    
                    ui.add_space(20.0);
                    
                    // 鍏ㄩ€?鍙栨秷鎸夐挳
                    ui.horizontal(|ui| {
                        if ui.button("鍏ㄩ€?).clicked() {
                            self.select_all(true);
                        }
                        if ui.button("鍙栨秷鍏ㄩ€?).clicked() {
                            self.select_all(false);
                        }
                    });
                });
                
                ui.separator();
                
                // 鍙充晶椤圭洰鍒楄〃
                ui.vertical(|ui| {
                    self.draw_categories(ui);
                });
            });
        });

        // 纭瀵硅瘽妗?        if self.show_confirm_dialog {
            self.draw_confirm_dialog(ctx);
        }
        
        // 缁撴灉瀵硅瘽妗?        if self.show_result_dialog {
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
            
            // 鏇存柊瀵瑰簲鍒嗙被鐨勯」鐩姸鎬?            for data_item in scanned {
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
            message: "鎵弿瀹屾垚".to_string(),
            item_path: None,
        });
    }

    fn draw_categories(&mut self, ui: &mut egui::Ui) {
        ui.heading("鍙竻鐞嗛」鐩?);
        ui.separator();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for cat_idx in 0..self.categories.len() {
                let category = &mut self.categories[cat_idx];
                
                // 鍒嗙被鏍囬鏍?                let header_response = egui::CollapsingHeader::new(
                    egui::RichText::new(format!("{} ({}/{})", 
                        category.name, 
                        category.selected_count, 
                        category.total_count
                    )).size(16.0).strong()
                )
                .default_open(category.expanded)
                .show(ui, |ui| {
                    // 鍒嗙被鍏ㄩ€夋寜閽?                    ui.horizontal(|ui| {
                        let all_selected = category.selected_count == category.total_count;
                        let mut select_all = all_selected;
                        if ui.checkbox(&mut select_all, "鍏ㄩ€夋鍒嗙被").changed() {
                            for item in &mut category.items {
                                item.selected = select_all;
                            }
                            category.selected_count = if select_all { category.total_count } else { 0 };
                            self.update_selected_count();
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // 椤圭洰鍒楄〃
                    for item_idx in 0..category.items.len() {
                        let item = &mut category.items[item_idx];
                        
                        let risk_color = match item.risk_level {
                            RiskLevel::Critical => egui::Color32::RED,
                            RiskLevel::High => egui::Color32::from_rgb(255, 140, 0),
                            RiskLevel::Medium => egui::Color32::from_rgb(255, 193, 7),
                            RiskLevel::Low => egui::Color32::from_rgb(40, 167, 69),
                        };
                        
                        let risk_text = match item.risk_level {
                            RiskLevel::Critical => "涓ラ噸",
                            RiskLevel::High => "楂橀闄?,
                            RiskLevel::Medium => "涓瓑",
                            RiskLevel::Low => "浣庨闄?,
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
                                                ui.colored_label(egui::Color32::GREEN, "宸插彂鐜?);
                                            }
                                        });
                                        
                                        ui.label(egui::RichText::new(&item.description).size(12.0).color(egui::Color32::DARK_GRAY));
                                        
                                        if item.scanned && item.size > 0 {
                                            ui.label(egui::RichText::new(format!("璺緞: {} | 澶у皬: {}", 
                                                item.path, Self::format_size(item.size)))
                                                .size(11.0).color(egui::Color32::GRAY));
                                        } else {
                                            ui.label(egui::RichText::new(format!("璺緞: {}", item.path))
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
        
        egui::Window::new("纭娓呯悊")
            .collapsible(false)
            .resizable(false)
            .fixed_size([450.0, 250.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(15.0);
                    ui.colored_label(
                        egui::Color32::RED,
                        egui::RichText::new("鈿?璀﹀憡").size(24.0).strong()
                    );
                    ui.add_space(15.0);
                    ui.label(egui::RichText::new("鍗冲皢姘镐箙鍒犻櫎浠ヤ笅鏁版嵁:").size(16.0));
                    ui.add_space(10.0);
                    
                    // 鏄剧ず瑕佸垹闄ょ殑椤圭洰鍒楄〃
                    let mut delete_list = Vec::new();
                    for category in &self.categories {
                        for item in &category.items {
                            if item.selected {
                                delete_list.push(format!("{} - {}", category.name, item.name));
                            }
                        }
                    }
                    
                    for item_name in &delete_list {
                        ui.label(egui::RichText::new(format!("鈥?{}", item_name)).size(14.0));
                    }
                    
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(format!("鍏?{} 涓」鐩?, self.selected_items)).size(14.0).strong());
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("鍒犻櫎鍚庢棤娉曟仮澶嶏紒").size(14.0).color(egui::Color32::RED));
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        let confirm_btn = ui.add_sized(
                            [130.0, 40.0],
                            egui::Button::new(
                                egui::RichText::new("纭鍒犻櫎").size(16.0).strong()
                            ).fill(egui::Color32::from_rgb(220, 53, 69))
                        );
                        if confirm_btn.clicked() {
                            self.execute_cleanup();
                            should_close = true;
                        }
                        
                        ui.add_space(30.0);
                        
                        if ui.add_sized([130.0, 40.0], egui::Button::new(
                            egui::RichText::new("鍙栨秷").size(16.0)
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
        
        egui::Window::new("娓呯悊瀹屾垚")
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
                            egui::RichText::new("娓呯悊鎴愬姛锛?).size(24.0).strong()
                        );
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 140, 0),
                            egui::RichText::new("娓呯悊瀹屾垚锛堥儴鍒嗗け璐ワ級").size(24.0).strong()
                        );
                    }
                    
                    ui.add_space(20.0);
                    ui.label(format!("鎴愬姛娓呯悊: {} 涓」鐩?, self.cleaned_count));
                    if self.failed_count > 0 {
                        ui.colored_label(egui::Color32::RED, format!("娓呯悊澶辫触: {} 涓」鐩?, self.failed_count));
                    }
                    ui.add_space(30.0);
                    
                    if ui.add_sized([120.0, 40.0], egui::Button::new(
                        egui::RichText::new("纭畾").size(16.0)
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
        
        // 鏀堕泦閫変腑鐨勯」鐩?        let mut items_to_clean: Vec<DataItem> = Vec::new();
        for category in &self.categories {
            for item in &category.items {
                if item.selected && item.scanned {
                    items_to_clean.push(DataItem {
                        id: item.id.clone(),
                        path: item.path.clone(),
                        data_type: models::DataType::Document,
                        risk_level: item.risk_level.clone(),
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
                
                // 鏍囪宸叉竻鐞嗙殑椤圭洰
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
                    message: format!("鎴愬姛娓呯悊 {} 涓」鐩?, self.cleaned_count),
                    item_path: None,
                });
            }
            Err(e) => {
                self.failed_count = items_to_clean.len();
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Error,
                    message: format!("娓呯悊澶辫触: {}", e),
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
