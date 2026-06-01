use super::Scanner;
use crate::models::{DataItem, DataType, RiskLevel};
use std::fs;
use std::path::Path;

pub struct VSCodeScanner;

impl Scanner for VSCodeScanner {
    fn name(&self) -> &str {
        "VSCode"
    }

    fn scan(&self) -> Vec<DataItem> {
        let mut items = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            let vscode_dirs = if cfg!(target_os = "windows") {
                vec![
                    home_dir.join("AppData").join("Roaming").join("Code"),
                    home_dir.join(".vscode"),
                ]
            } else if cfg!(target_os = "macos") {
                vec![
                    home_dir.join("Library").join("Application Support").join("Code"),
                    home_dir.join(".vscode"),
                ]
            } else {
                vec![
                    home_dir.join(".config").join("Code"),
                    home_dir.join(".vscode"),
                ]
            };

            for dir in vscode_dirs {
                if dir.exists() {
                    self.scan_vscode_dir(&dir, &mut items);
                }
            }
        }

        items
    }
}

impl VSCodeScanner {
    fn scan_vscode_dir(&self, dir: &Path, items: &mut Vec<DataItem>) {
        if let Some(dir_name) = dir.file_name() {
            let dir_str = dir_name.to_string_lossy();
            
            if dir_str == ".vscode" {
                self.scan_dot_vscode(dir, items);
            } else if dir_str == "Code" {
                self.scan_code_dir(dir, items);
            }
        }
    }

    fn scan_dot_vscode(&self, dir: &Path, items: &mut Vec<DataItem>) {
        let extensions_path = dir.join("extensions");
        if extensions_path.exists() && extensions_path.is_dir() {
            if let Some(item) = self.create_data_item(&extensions_path, "Extensions Directory") {
                items.push(item);
            }
        }
    }

    fn scan_code_dir(&self, dir: &Path, items: &mut Vec<DataItem>) {
        let user_dir = dir.join("User");
        if user_dir.exists() && user_dir.is_dir() {
            self.scan_user_dir(&user_dir, items);
        }

        let workspace_storage = dir.join("User").join("workspaceStorage");
        if workspace_storage.exists() && workspace_storage.is_dir() {
            if let Some(item) = self.create_data_item(&workspace_storage, "Workspace Storage") {
                items.push(item);
            }
        }

        let cached_data = dir.join("CachedData");
        if cached_data.exists() && cached_data.is_dir() {
            if let Some(item) = self.create_data_item(&cached_data, "Cached Data") {
                items.push(item);
            }
        }

        let cache = dir.join("Cache");
        if cache.exists() && cache.is_dir() {
            if let Some(item) = self.create_data_item(&cache, "Cache") {
                items.push(item);
            }
        }
    }

    fn scan_user_dir(&self, user_dir: &Path, items: &mut Vec<DataItem>) {
        let target_files = vec![
            "settings.json",
            "keybindings.json",
            "tasks.json",
            "launch.json",
            "extensions.json",
            "snippets",
        ];

        for file in target_files {
            let file_path = user_dir.join(file);
            if file_path.exists() {
                if let Some(item) = self.create_data_item(&file_path, "User Configuration") {
                    items.push(item);
                }
            }
        }

        let global_storage = user_dir.join("globalStorage");
        if global_storage.exists() && global_storage.is_dir() {
            if let Some(item) = self.create_data_item(&global_storage, "Global Storage") {
                items.push(item);
            }
        }
    }

    fn create_data_item(&self, path: &Path, category: &str) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = if metadata.is_dir() {
                self.calculate_dir_size(path)
            } else {
                metadata.len()
            };
            
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            
            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("vscode-{}-{}", category, file_name);
            
            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                if metadata.is_dir() { DataType::Other } else { DataType::Code },
                RiskLevel::Medium,
                size,
                created_at,
                modified_at,
            );
            
            item.description = Some(format!(
                "VSCode - {}: {}",
                category,
                file_name
            ));
            
            Some(item)
        } else {
            None
        }
    }

    fn calculate_dir_size(&self, dir: &Path) -> u64 {
        let mut size = 0;
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.is_dir() {
                        size += self.calculate_dir_size(&path);
                    } else {
                        size += metadata.len();
                    }
                }
            }
        }
        
        size
    }
}
