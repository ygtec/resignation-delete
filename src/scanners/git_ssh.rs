use super::Scanner;
use crate::models::{DataItem, DataType, RiskLevel};
use std::fs;
use std::path::{Path, PathBuf};

pub struct GitSshScanner;

impl Scanner for GitSshScanner {
    fn name(&self) -> &str {
        "Git SSH Keys"
    }

    fn scan(&self) -> Vec<DataItem> {
        let mut items = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            let ssh_dir = home_dir.join(".ssh");

            if ssh_dir.exists() && ssh_dir.is_dir() {
                self.scan_ssh_dir(&ssh_dir, &mut items);
            }
        }

        items
    }
}

impl GitSshScanner {
    fn scan_ssh_dir(&self, dir: &Path, items: &mut Vec<DataItem>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    self.process_ssh_file(&path, items);
                }
            }
        }
    }

    fn process_ssh_file(&self, path: &Path, items: &mut Vec<DataItem>) {
        if let Some(ext) = path.extension() {
            if ext == "pub" || ext == "pem" || ext == "key" {
                if let Some(item) = self.create_data_item(path, "SSH Key") {
                    items.push(item);
                }
            }
        } else {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.starts_with("id_") {
                        if let Some(item) = self.create_data_item(path, "SSH Private Key") {
                            items.push(item);
                        }
                    }
                }
            }
        }
    }

    fn create_data_item(&self, path: &Path, category: &str) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("git-ssh-{}-{}", category.to_lowercase().replace(' ', "-"), file_name);

            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                DataType::Other,
                RiskLevel::High,
                size,
                created_at,
                modified_at,
            );

            item.description = Some(format!("Git SSH - {}: {}", category, file_name));

            Some(item)
        } else {
            None
        }
    }
}
