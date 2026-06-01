use super::Scanner;
use crate::models::{DataItem, DataType, RiskLevel};
use std::fs;
use std::path::{Path, PathBuf};

pub struct JetBrainsScanner;

impl Scanner for JetBrainsScanner {
    fn name(&self) -> &str {
        "JetBrains IDEs"
    }

    fn scan(&self) -> Vec<DataItem> {
        let mut items = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            let jetbrains_dir = if cfg!(target_os = "windows") {
                home_dir.join("AppData").join("Roaming").join("JetBrains")
            } else if cfg!(target_os = "macos") {
                home_dir.join("Library").join("Application Support").join("JetBrains")
            } else {
                home_dir.join(".config").join("JetBrains")
            };

            if jetbrains_dir.exists() {
                self.scan_jetbrains_dir(&jetbrains_dir, &mut items);
            }

            let old_jetbrains_dir = if cfg!(target_os = "windows") {
                home_dir.join(".IntelliJIdea*")
            } else {
                home_dir.join(".IntelliJIdea*")
            };

            items.extend(self.scan_legacy_jetbrains_dirs(&home_dir));
        }

        items
    }
}

impl JetBrainsScanner {
    fn scan_jetbrains_dir(&self, dir: &Path, items: &mut Vec<DataItem>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();
                    
                    let config_path = path.join("options");
                    self.scan_config_dir(&config_path, &dir_name, items);
                    
                    let eval_path = path.join("eval");
                    self.scan_eval_dir(&eval_path, &dir_name, items);
                }
            }
        }
    }

    fn scan_legacy_jetbrains_dirs(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();
        
        let ide_dirs = vec![
            ".IntelliJIdea*",
            ".PyCharm*",
            ".WebStorm*",
            ".PhpStorm*",
            ".RubyMine*",
            ".GoLand*",
            ".CLion*",
            ".Rider*",
            ".DataGrip*",
            ".AndroidStudio*",
        ];

        for pattern in ide_dirs {
            if let Ok(entries) = glob::glob(home_dir.join(pattern).to_str().unwrap_or("")) {
                for entry in entries.flatten() {
                    if entry.is_dir() {
                        let dir_name = entry.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let config_path = entry.join("config").join("options");
                        self.scan_config_dir(&config_path, &dir_name, &mut items);
                    }
                }
            }
        }

        items
    }

    fn scan_config_dir(&self, config_dir: &Path, ide_name: &str, items: &mut Vec<DataItem>) {
        if !config_dir.exists() {
            return;
        }

        let target_files = vec![
            "other.xml",
            "keymaps.xml",
            "colors.scheme.xml",
            "editor.xml",
            "filetypes.xml",
            "recentProjects.xml",
            "ui.lnf.xml",
        ];

        for file in target_files {
            let file_path = config_dir.join(file);
            if file_path.exists() {
                if let Some(item) = self.create_data_item(&file_path, ide_name, "IDE Configuration") {
                    items.push(item);
                }
            }
        }
    }

    fn scan_eval_dir(&self, eval_dir: &Path, ide_name: &str, items: &mut Vec<DataItem>) {
        if !eval_dir.exists() {
            return;
        }

        if let Ok(entries) = fs::read_dir(eval_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(item) = self.create_data_item(&path, ide_name, "License/Evaluation") {
                        items.push(item);
                    }
                }
            }
        }
    }

    fn create_data_item(&self, path: &Path, ide_name: &str, category: &str) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            
            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("jetbrains-{}-{}-{}", ide_name, category, file_name);
            
            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                DataType::Other,
                RiskLevel::Medium,
                size,
                created_at,
                modified_at,
            );
            
            item.description = Some(format!(
                "{} - {}: {}",
                ide_name,
                category,
                file_name
            ));
            
            Some(item)
        } else {
            None
        }
    }
}
