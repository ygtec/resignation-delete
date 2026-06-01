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
            .with_inner_size([1100.0, 750.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Resignation Delete",
        options,
        Box::new(|cc| {
            configure_fonts(&cc.egui_ctx);
            Ok(Box::<ResignationDeleteApp>::default())
        }),
    )
}

fn configure_fonts(ctx: &egui::Context) {
    #[cfg(target_os = "windows")]
    {
        let mut fonts = egui::FontDefinitions::default();
        
        if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/segoeui.ttf") {
            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_owned(font_data).into(),
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
    }
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
    scan_progress: f32,
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
            scan_progress: 0.0,
        }
    }
}

impl eframe::App for ResignationDeleteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.heading("Resignation Delete - Personal Data Cleanup Tool");
            });
            ui.separator();
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.is_scanning {
                    ui.add(egui::ProgressBar::new(self.scan_progress).show_percentage());
                    ui.label("Scanning system...");
                } else if self.is_cleaning {
                    ui.label("Cleaning data... Please wait");
                } else if !self.scanned_items.is_empty() {
                    ui.label(format!(
                        "Found: {} items | Selected: {} | Ready",
                        self.scanned_items.len(),
                        self.selected_items.len()
                    ));
                } else {
                    ui.label("Click 'Scan System' to start");
                }
            });
        });

        egui::SidePanel::left("control_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Actions");
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    
                    let scan_btn = ui.add_sized(
                        [180.0, 40.0],
                        egui::Button::new("Scan System")
                            .fill(egui::Color32::from_rgb(0, 120, 212))
                    );
                    if scan_btn.clicked() && !self.is_scanning {
                        self.perform_scan();
                    }
                    
                    ui.add_space(10.0);
                    
                    ui.add_enabled_ui(!self.scanned_items.is_empty(), |ui| {
                        if ui.add_sized([180.0, 35.0], egui::Button::new("Select All")).clicked() {
                            self.selected_items.clear();
                            for item in &self.scanned_items {
                                self.selected_items.insert(item.id.clone());
                            }
                        }
                        
                        if ui.add_sized([180.0, 35.0], egui::Button::new("Deselect All")).clicked() {
                            self.selected_items.clear();
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    ui.add_enabled_ui(!self.selected_items.is_empty() && !self.is_cleaning, |ui| {
                        let clean_btn = ui.add_sized(
                            [180.0, 40.0],
                            egui::Button::new("Clean Selected")
                                .fill(egui::Color32::from_rgb(220, 53, 69))
                        );
                        if clean_btn.clicked() {
                            self.show_confirm_dialog = true;
                        }
                    });
                    
                    ui.add_space(30.0);
                    ui.heading("Statistics");
                    ui.separator();
                    
                    if !self.scanned_items.is_empty() {
                        let total = self.scanned_items.len();
                        let selected = self.selected_items.len();
                        let critical = self.scanned_items.iter()
                            .filter(|i| self.selected_items.contains(&i.id) && i.risk_level == RiskLevel::Critical)
                            .count();
                        let high = self.scanned_items.iter()
                            .filter(|i| self.selected_items.contains(&i.id) && i.risk_level == RiskLevel::High)
                            .count();
                        
                        ui.label(format!("Total: {}", total));
                        ui.label(format!("Selected: {}", selected));
                        if critical > 0 {
                            ui.colored_label(egui::Color32::RED, format!("Critical: {}", critical));
                        }
                        if high > 0 {
                            ui.colored_label(egui::Color32::from_rgb(255, 165, 0), format!("High Risk: {}", high));
                        }
                    } else {
                        ui.label("No data scanned yet");
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_item_list(ui);
        });

        if self.show_confirm_dialog {
            self.draw_confirm_dialog(ctx);
        }
    }
}

impl ResignationDeleteApp {
    fn perform_scan(&mut self) {
        self.is_scanning = true;
        self.scan_progress = 0.0;
        self.scanned_items.clear();
        self.selected_items.clear();
        
        let scanner_count = self.scanners.len() as f32;
        for (i, scanner) in self.scanners.iter().enumerate() {
            self.scan_progress = (i as f32) / scanner_count;
            self.scanned_items.extend(scanner.scan());
        }
        
        self.scan_progress = 1.0;
        self.is_scanning = false;
        
        self.logs.push(CleanLog {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: LogLevel::Info,
            message: format!("Scan complete. Found {} items.", self.scanned_items.len()),
            item_path: None,
        });
    }

    fn draw_item_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Scanned Items");
        ui.separator();

        if self.scanned_items.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.label("No items scanned yet.");
                ui.label("Click 'Scan System' to find personal data.");
            });
            return;
        }

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
        
        egui::ScrollArea::vertical().show_rows(
            ui,
            text_height + 8.0,
            self.scanned_items.len(),
            |ui, row_range| {
                for row in row_range {
                    let item = &self.scanned_items[row];
                    let is_selected = self.selected_items.contains(&item.id);
                    
                    let (bg_color, text_color) = if is_selected {
                        (egui::Color32::from_rgb(230, 245, 255), egui::Color32::BLACK)
                    } else {
                        (egui::Color32::WHITE, egui::Color32::BLACK)
                    };
                    
                    let risk_color = match item.risk_level {
                        RiskLevel::Critical => egui::Color32::RED,
                        RiskLevel::High => egui::Color32::from_rgb(255, 140, 0),
                        RiskLevel::Medium => egui::Color32::from_rgb(255, 193, 7),
                        RiskLevel::Low => egui::Color32::from_rgb(40, 167, 69),
                    };
                    
                    let risk_text = match item.risk_level {
                        RiskLevel::Critical => "CRITICAL",
                        RiskLevel::High => "HIGH",
                        RiskLevel::Medium => "MEDIUM",
                        RiskLevel::Low => "LOW",
                    };
                    
                    egui::Frame::group(ui.style())
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let mut checked = is_selected;
                                if ui.checkbox(&mut checked, "").changed() {
                                    if checked {
                                        self.selected_items.insert(item.id.clone());
                                    } else {
                                        self.selected_items.remove(&item.id);
                                    }
                                }
                                
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(&item.id).strong().color(text_color));
                                        ui.colored_label(risk_color, format!("[{}]", risk_text));
                                    });
                                    
                                    if let Some(desc) = &item.description {
                                        ui.label(egui::RichText::new(desc).size(12.0).color(egui::Color32::DARK_GRAY));
                                    }
                                    
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("Path: {}", item.path)).size(11.0).color(egui::Color32::GRAY));
                                        ui.label(egui::RichText::new(format!("Size: {}", Self::format_size(item.size))).size(11.0).color(egui::Color32::GRAY));
                                    });
                                });
                            });
                        });
                    
                    ui.add_space(4.0);
                }
            },
        );
    }

    fn draw_confirm_dialog(&mut self, ctx: &egui::Context) {
        let mut should_close = false;
        
        egui::Window::new("Confirm Deletion")
            .collapsible(false)
            .resizable(false)
            .fixed_size([400.0, 200.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.colored_label(
                        egui::Color32::RED,
                        egui::RichText::new("WARNING").size(20.0).strong()
                    );
                    ui.add_space(10.0);
                    ui.label("This action will permanently delete selected data!");
                    ui.label("Deleted data cannot be recovered.");
                    ui.add_space(10.0);
                    ui.label(format!("Items to delete: {}", self.selected_items.len()));
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        let confirm_btn = ui.add_sized(
                            [120.0, 35.0],
                            egui::Button::new("Confirm Delete")
                                .fill(egui::Color32::from_rgb(220, 53, 69))
                        );
                        if confirm_btn.clicked() {
                            self.execute_cleanup();
                            should_close = true;
                        }
                        
                        ui.add_space(20.0);
                        
                        if ui.add_sized([120.0, 35.0], egui::Button::new("Cancel")).clicked() {
                            should_close = true;
                        }
                    });
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
                    message: "Cleanup completed successfully".to_string(),
                    item_path: None,
                });
                
                self.scanned_items.retain(|item| !self.selected_items.contains(&item.id));
                self.selected_items.clear();
            }
            Err(e) => {
                self.logs.push(CleanLog {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    level: LogLevel::Error,
                    message: format!("Cleanup failed: {}", e),
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
