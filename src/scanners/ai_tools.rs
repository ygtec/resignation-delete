use super::Scanner;
use crate::models::{DataItem, DataType, RiskLevel};
use std::fs;
use std::path::Path;

pub struct AIToolsScanner;

impl Scanner for AIToolsScanner {
    fn name(&self) -> &str {
        "AI Tools"
    }

    fn scan(&self) -> Vec<DataItem> {
        let mut items = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            items.extend(self.scan_cursor(&home_dir));
            items.extend(self.scan_claude(&home_dir));
            items.extend(self.scan_kimi(&home_dir));
            items.extend(self.scan_qwen(&home_dir));
            items.extend(self.scan_github_copilot(&home_dir));
            items.extend(self.scan_opencode(&home_dir));
            items.extend(self.scan_trae(&home_dir));
        }

        items
    }
}

impl AIToolsScanner {
    fn scan_cursor(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let cursor_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("Cursor"),
                home_dir.join(".cursor"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("Cursor"),
                home_dir.join(".cursor"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("Cursor"),
                home_dir.join(".cursor"),
            ]
        };

        for dir in cursor_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "Cursor", "Configuration") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn scan_claude(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let claude_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("Claude"),
                home_dir.join("AppData").join("Local").join("Claude"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("Claude"),
                home_dir.join("Library").join("Caches").join("Claude"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("claude"),
                home_dir.join(".cache").join("claude"),
            ]
        };

        for dir in claude_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "Claude", "Configuration/Cache") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn scan_kimi(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let kimi_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("Kimi"),
                home_dir.join("AppData").join("Local").join("Kimi"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("Kimi"),
                home_dir.join("Library").join("Caches").join("Kimi"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("kimi"),
                home_dir.join(".cache").join("kimi"),
            ]
        };

        for dir in kimi_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "Kimi", "Configuration/Cache") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn scan_qwen(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let qwen_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("Qwen"),
                home_dir.join("AppData").join("Local").join("Qwen"),
                home_dir.join("AppData").join("Roaming").join("Tongyi"),
                home_dir.join("AppData").join("Local").join("Tongyi"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("Qwen"),
                home_dir.join("Library").join("Caches").join("Qwen"),
                home_dir.join("Library").join("Application Support").join("Tongyi"),
                home_dir.join("Library").join("Caches").join("Tongyi"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("qwen"),
                home_dir.join(".cache").join("qwen"),
                home_dir.join(".config").join("tongyi"),
                home_dir.join(".cache").join("tongyi"),
            ]
        };

        for dir in qwen_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "Qwen/Tongyi", "Configuration/Cache") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn scan_github_copilot(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let copilot_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("GitHub Copilot"),
                home_dir.join("AppData").join("Local").join("GitHub Copilot"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("GitHub Copilot"),
                home_dir.join("Library").join("Caches").join("GitHub Copilot"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("github-copilot"),
                home_dir.join(".cache").join("github-copilot"),
            ]
        };

        for dir in copilot_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "GitHub Copilot", "Configuration/Cache") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn scan_opencode(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let opencode_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("OpenCode"),
                home_dir.join("AppData").join("Local").join("OpenCode"),
                home_dir.join(".opencode"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("OpenCode"),
                home_dir.join("Library").join("Caches").join("OpenCode"),
                home_dir.join(".opencode"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("opencode"),
                home_dir.join(".cache").join("opencode"),
                home_dir.join(".opencode"),
            ]
        };

        for dir in opencode_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "OpenCode", "Configuration/Cache") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn scan_trae(&self, home_dir: &Path) -> Vec<DataItem> {
        let mut items = Vec::new();

        let trae_dirs = if cfg!(target_os = "windows") {
            vec![
                home_dir.join("AppData").join("Roaming").join("Trae"),
                home_dir.join("AppData").join("Local").join("Trae"),
                home_dir.join(".trae"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                home_dir.join("Library").join("Application Support").join("Trae"),
                home_dir.join("Library").join("Caches").join("Trae"),
                home_dir.join(".trae"),
            ]
        } else {
            vec![
                home_dir.join(".config").join("trae"),
                home_dir.join(".cache").join("trae"),
                home_dir.join(".trae"),
            ]
        };

        for dir in trae_dirs {
            if dir.exists() {
                if let Some(item) = self.create_data_item(&dir, "Trae", "Configuration/Cache") {
                    items.push(item);
                }
            }
        }

        items
    }

    fn create_data_item(&self, path: &Path, tool_name: &str, category: &str) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = if metadata.is_dir() {
                self.calculate_dir_size(path)
            } else {
                metadata.len()
            };
            
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            
            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("ai-{}-{}-{}", tool_name.to_lowercase(), category.to_lowercase(), file_name);
            
            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                DataType::Other,
                RiskLevel::High,
                size,
                created_at,
                modified_at,
            );
            
            item.description = Some(format!(
                "{} - {}: {}",
                tool_name,
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
