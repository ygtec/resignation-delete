use super::Scanner;
use crate::models::{DataItem, DataType, RiskLevel};
use std::fs;
use std::path::{Path, PathBuf};

pub struct BrowsersScanner;

impl Scanner for BrowsersScanner {
    fn name(&self) -> &str {
        "Browsers"
    }

    fn scan(&self) -> Vec<DataItem> {
        let mut items = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            self.scan_chrome(&home_dir, &mut items);
            self.scan_edge(&home_dir, &mut items);
            self.scan_firefox(&home_dir, &mut items);
        }

        items
    }
}

impl BrowsersScanner {
    fn scan_chrome(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let chrome_path = if cfg!(target_os = "windows") {
            home_dir.join("AppData").join("Local").join("Google").join("Chrome").join("User Data")
        } else if cfg!(target_os = "macos") {
            home_dir.join("Library").join("Application Support").join("Google").join("Chrome")
        } else {
            home_dir.join(".config").join("google-chrome")
        };

        self.scan_chromium_browser(&chrome_path, "Chrome", items);
    }

    fn scan_edge(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let edge_path = if cfg!(target_os = "windows") {
            home_dir.join("AppData").join("Local").join("Microsoft").join("Edge").join("User Data")
        } else if cfg!(target_os = "macos") {
            home_dir.join("Library").join("Application Support").join("Microsoft Edge")
        } else {
            home_dir.join(".config").join("microsoft-edge")
        };

        self.scan_chromium_browser(&edge_path, "Edge", items);
    }

    fn scan_firefox(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let firefox_path = if cfg!(target_os = "windows") {
            home_dir.join("AppData").join("Roaming").join("Mozilla").join("Firefox").join("Profiles")
        } else if cfg!(target_os = "macos") {
            home_dir.join("Library").join("Application Support").join("Firefox").join("Profiles")
        } else {
            home_dir.join(".mozilla").join("firefox")
        };

        if firefox_path.exists() && firefox_path.is_dir() {
            if let Ok(entries) = fs::read_dir(firefox_path) {
                for entry in entries.flatten() {
                    let profile_path = entry.path();
                    if profile_path.is_dir() {
                        self.add_firefox_profile_items(&profile_path, items);
                    }
                }
            }
        }
    }

    fn scan_chromium_browser(&self, base_path: &Path, browser_name: &str, items: &mut Vec<DataItem>) {
        if !base_path.exists() || !base_path.is_dir() {
            return;
        }

        let profile_names = vec!["Default", "Profile 1", "Profile 2", "Profile 3", "Profile 4", "Profile 5"];

        for profile_name in profile_names {
            let profile_path = base_path.join(profile_name);
            if profile_path.exists() && profile_path.is_dir() {
                self.add_chromium_profile_items(&profile_path, browser_name, profile_name, items);
            }
        }
    }

    fn add_chromium_profile_items(&self, profile_path: &Path, browser_name: &str, profile_name: &str, items: &mut Vec<DataItem>) {
        let files = vec![
            ("History", "Browser History", RiskLevel::High),
            ("Cookies", "Cookies", RiskLevel::High),
            ("Login Data", "Login Credentials", RiskLevel::Critical),
            ("Bookmarks", "Bookmarks", RiskLevel::Medium),
            ("Preferences", "Preferences", RiskLevel::Medium),
            ("Web Data", "Web Data", RiskLevel::High),
        ];

        for (file, description, risk_level) in files {
            let file_path = profile_path.join(file);
            if file_path.exists() && file_path.is_file() {
                if let Some(item) = self.create_data_item(&file_path, browser_name, profile_name, description, risk_level) {
                    items.push(item);
                }
            }
        }
    }

    fn add_firefox_profile_items(&self, profile_path: &Path, items: &mut Vec<DataItem>) {
        let profile_name = profile_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown Profile")
            .to_string();

        let files = vec![
            ("places.sqlite", "History and Bookmarks", RiskLevel::High),
            ("cookies.sqlite", "Cookies", RiskLevel::High),
            ("logins.json", "Login Credentials", RiskLevel::Critical),
            ("formhistory.sqlite", "Form History", RiskLevel::Medium),
            ("key4.db", "Password Database", RiskLevel::Critical),
        ];

        for (file, description, risk_level) in files {
            let file_path = profile_path.join(file);
            if file_path.exists() && file_path.is_file() {
                if let Some(item) = self.create_firefox_data_item(&file_path, &profile_name, description, risk_level) {
                    items.push(item);
                }
            }
        }
    }

    fn create_data_item(&self, path: &Path, browser_name: &str, profile_name: &str, description: &str, risk_level: RiskLevel) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("browser-{}-{}-{}", browser_name.to_lowercase(), profile_name.to_lowercase().replace(' ', "-"), file_name);

            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                DataType::Other,
                risk_level,
                size,
                created_at,
                modified_at,
            );

            item.description = Some(format!("{} - {} ({})", browser_name, description, profile_name));

            Some(item)
        } else {
            None
        }
    }

    fn create_firefox_data_item(&self, path: &Path, profile_name: &str, description: &str, risk_level: RiskLevel) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("browser-firefox-{}-{}", profile_name, file_name);

            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                DataType::Other,
                risk_level,
                size,
                created_at,
                modified_at,
            );

            item.description = Some(format!("Firefox - {} ({})", description, profile_name));

            Some(item)
        } else {
            None
        }
    }
}
