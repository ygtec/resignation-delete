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
        initial_window_size: Some(egui::vec2(1000.0, 700.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Resignation Delete - 数据清理工具",
        options,
        Box::new(|_cc| Box::<ResignationDeleteApp>::default()),
    )
}

struct ResignationDeleteApp {
    scanners: Vec<Box<dyn Scanner>>,
    scanned_items: Vec<DataItem>,
    selected_items: HashSet<String>,
    show_confirm_dialog: bool,
    cleaner: Cleaner,
    logs: Vec<CleanLog>,
    is_scanning: bool,
    is_cleaning: bool,
}

impl Default for ResignationDeleteApp {
    fn default() -> Self {
        Self {
            scanners: vec![
                Box::new(GitSshScanner),
                Box::new(BrowsersScanner),
                Box::new(JetBrainsScanner),
                Box::new(VSCodeScanner),
                Box::new(AIToolsScanner),
            ],
            scanned_items: Vec::new(),
            selected_items: HashSet::new(),
            show_confirm_dialog: false,
            cleaner: Cleaner::new(),
            logs: Vec::new(),
            is_scanning: false,
            is_cleaning: false,
        }
    }
}

impl eframe::App for ResignationDeleteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Resignation Delete - 离职数据清理工具");
            ui.add_space(10.0);

            self.draw_control_panel(ui);
            ui.add_space(10.0);

            self.draw_risk_warning(ui);
            ui.add_space(10.0);

            self.draw_item_list(ui);
            ui.add_space(10.0);

            self.draw_log_panel(ui);
        });

        if self.show_confirm_dialog {
            self.draw_confirm_dialog(ctx);
        }
    }
}

impl ResignationDeleteApp {
    fn draw_control_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔍 扫描系统").clicked() && !self.is_scanning {
                self.is_scanning = true;
                self.scanned_items.clear();
                self.selected_items.clear();
                
                for scanner in &self.scanners {
                    self.scanned_items.extend(scanner.scan());
                }
                
                self.is_scanning = false;
                
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Info,
                    message: format!("扫描完成，发现 {} 个数据项", self.scanned_items.len()),
                    item_path: None,
                });
            }

            ui.add_enabled_ui(!self.scanned_items.is_empty(), |ui| {
                if ui.button("✅ 全选").clicked() {
                    self.selected_items.clear();
                    for item in &self.scanned_items {
                        self.selected_items.insert(item.id.clone());
                    }
                }
                
                if ui.button("❌ 取消全选").clicked() {
                    self.selected_items.clear();
                }
            });

            ui.add_space(20.0);
            
            ui.add_enabled_ui(!self.selected_items.is_empty() && !self.is_cleaning, |ui| {
                if ui.button("🗑️ 清除选中项").clicked() {
                    self.show_confirm_dialog = true;
                }
            });
        });

        if self.is_scanning {
            ui.label("正在扫描系统...");
        } else if !self.scanned_items.is_empty() {
            ui.label(format!("已扫描 {} 个数据项，选中 {} 个", self.scanned_items.len(), self.selected_items.len()));
        }
    }

    fn draw_risk_warning(&self, ui: &mut egui::Ui) {
        if !self.selected_items.is_empty() {
            let critical_count = self.selected_items.iter()
                .filter(|&id| self.scanned_items.iter().any(|item| item.id == *id && item.risk_level == RiskLevel::Critical))
                .count();
            let high_count = self.selected_items.iter()
                .filter(|&id| self.scanned_items.iter().any(|item| item.id == *id && item.risk_level == RiskLevel::High))
                .count();
            
            if critical_count > 0 || high_count > 0 {
                ui.group(|ui| {
                    ui.colored_label(egui::Color32::RED, "⚠️ 高风险警告");
                    if critical_count > 0 {
                        ui.label(format!("• 包含 {} 个严重风险项 (Critical)", critical_count));
                    }
                    if high_count > 0 {
                        ui.label(format!("• 包含 {} 个高风险项 (High)", high_count));
                    }
                    ui.label("这些数据项包含敏感信息，请谨慎操作！");
                });
            }
        }
    }

    fn draw_item_list(&mut self, ui: &mut egui::Ui) {
        if self.scanned_items.is_empty() {
            ui.label("点击\"扫描系统\"开始查找可清理的数据项。");
            return;
        }

        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            let items_clone = self.scanned_items.clone();
            for item in items_clone {
                let mut is_selected = self.selected_items.contains(&item.id);
                
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut is_selected, "").changed() {
                        if is_selected {
                            self.selected_items.insert(item.id.clone());
                        } else {
                            self.selected_items.remove(&item.id);
                        }
                    }

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.strong(&item.id);
                                
                                let risk_color = match item.risk_level {
                                    RiskLevel::Critical => egui::Color32::RED,
                                    RiskLevel::High => egui::Color32::ORANGE,
                                    RiskLevel::Medium => egui::Color32::YELLOW,
                                    RiskLevel::Low => egui::Color32::GREEN,
                                };
                                let risk_label = match item.risk_level {
                                    RiskLevel::Critical => "Critical",
                                    RiskLevel::High => "High",
                                    RiskLevel::Medium => "Medium",
                                    RiskLevel::Low => "Low",
                                };
                                ui.colored_label(risk_color, format!("[{}]", risk_label));
                            });
                            
                            if let Some(desc) = &item.description {
                                ui.label(desc);
                            }
                            
                            ui.label(format!("路径: {}", item.path));
                            ui.label(format!("大小: {}", Self::format_size(item.size)));
                        });
                    });
                });
                
                ui.add_space(5.0);
            }
        });
    }

    fn draw_log_panel(&self, ui: &mut egui::Ui) {
        if !self.logs.is_empty() || !self.cleaner.logs().is_empty() {
            ui.collapsing("📋 操作日志", |ui| {
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    for log in self.cleaner.logs().iter().chain(self.logs.iter()) {
                        let color = match log.level {
                            LogLevel::Info => egui::Color32::LIGHT_GRAY,
                            LogLevel::Warning => egui::Color32::YELLOW,
                            LogLevel::Error => egui::Color32::RED,
                        };
                        
                        ui.colored_label(color, format!("[{}] {}", log.timestamp, log.message));
                    }
                });
            });
        }
    }

    fn draw_confirm_dialog(&mut self, ctx: &egui::Context) {
        let mut should_close = false;
        
        egui::Window::new("⚠️ 确认清除")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("此操作将永久删除选中的数据项！");
                ui.label("被删除的数据将无法恢复，请务必确认！");
                ui.add_space(10.0);
                
                ui.label(format!("即将清除 {} 个数据项", self.selected_items.len()));
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button("✅ 确认清除").clicked() {
                        self.execute_cleanup();
                        should_close = true;
                    }
                    
                    if ui.button("❌ 取消").clicked() {
                        should_close = true;
                    }
                });
            });
        
        if should_close {
            self.show_confirm_dialog = false;
        }
    }

    fn execute_cleanup(&mut self) {
        self.is_cleaning = true;
        
        let selected_items: Vec<DataItem> = self.scanned_items
            .iter()
            .filter(|item| self.selected_items.contains(&item.id))
            .cloned()
            .collect();
        
        self.cleaner.clear_tasks();
        self.cleaner.add_tasks(selected_items);
        
        let result = self.cleaner.clean_all(|_| true);
        
        match result {
            Ok(_) => {
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Info,
                    message: "清除操作完成".to_string(),
                    item_path: None,
                });
                
                self.scanned_items.retain(|item| !self.selected_items.contains(&item.id));
                self.selected_items.clear();
            }
            Err(e) => {
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Error,
                    message: format!("清除操作失败: {}", e),
                    item_path: None,
                });
            }
        }
        
        self.is_cleaning = false;
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
