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
            // 主流浏览器
            self.scan_chrome(&home_dir, &mut items);
            self.scan_edge(&home_dir, &mut items);
            self.scan_firefox(&home_dir, &mut items);
            self.scan_safari(&home_dir, &mut items);
            
            // 国内浏览器
            self.scan_qq_browser(&home_dir, &mut items);
            self.scan_360_browser(&home_dir, &mut items);
            self.scan_sogou(&home_dir, &mut items);
            self.scan_uc_browser(&home_dir, &mut items);
            self.scan_maxthon(&home_dir, &mut items);
            self.scan_cent(&home_dir, &mut items);
            
            // 国际浏览器
            self.scan_opera(&home_dir, &mut items);
            self.scan_opera_gx(&home_dir, &mut items);
            self.scan_brave(&home_dir, &mut items);
            self.scan_vivaldi(&home_dir, &mut items);
            self.scan_yandex(&home_dir, &mut items);
            self.scan_tor(&home_dir, &mut items);
            self.scan_waterfox(&home_dir, &mut items);
            self.scan_pale_moon(&home_dir, &mut items);
            self.scan_seamonkey(&home_dir, &mut items);
            
            // 跨境电商/指纹浏览器
            self.scan_thor(&home_dir, &mut items);
            self.scan_adspower(&home_dir, &mut items);
            self.scan_multilogin(&home_dir, &mut items);
            self.scan_lalicat(&home_dir, &mut items);
            self.scan_ixbrowser(&home_dir, &mut items);
            self.scan_vmlogin(&home_dir, &mut items);
            self.scan_hubstudio(&home_dir, &mut items);
            self.scan_dolphin(&home_dir, &mut items);
            self.scan_morelogin(&home_dir, &mut items);
            self.scan_gologin(&home_dir, &mut items);
            self.scan_incogniton(&home_dir, &mut items);
            self.scan_clonbrowser(&home_dir, &mut items);
            self.scan_octobrowser(&home_dir, &mut items);
        }

        items
    }
}

impl BrowsersScanner {
    // Chrome
    fn scan_chrome(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = self.get_browser_paths(home_dir, "Google", "Chrome", "google-chrome");
        for path in paths {
            self.scan_chromium_browser(&path, "Chrome", items);
        }
    }

    // Edge
    fn scan_edge(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = self.get_browser_paths(home_dir, "Microsoft", "Edge", "microsoft-edge");
        for path in paths {
            self.scan_chromium_browser(&path, "Edge", items);
        }
    }

    // Firefox
    fn scan_firefox(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let firefox_path = if cfg!(target_os = "windows") {
            home_dir.join("AppData").join("Roaming").join("Mozilla").join("Firefox").join("Profiles")
        } else if cfg!(target_os = "macos") {
            home_dir.join("Library").join("Application Support").join("Firefox").join("Profiles")
        } else {
            home_dir.join(".mozilla").join("firefox")
        };

        if firefox_path.exists() && firefox_path.is_dir() {
            if let Ok(entries) = fs::read_dir(&firefox_path) {
                for entry in entries.flatten() {
                    let profile_path = entry.path();
                    if profile_path.is_dir() {
                        self.add_firefox_profile_items(&profile_path, items);
                    }
                }
            }
        }
    }

    // Safari (macOS only)
    fn scan_safari(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        if cfg!(target_os = "macos") {
            let safari_paths = vec![
                home_dir.join("Library").join("Safari"),
                home_dir.join("Library").join("Caches").join("com.apple.Safari"),
                home_dir.join("Library").join("WebKit"),
            ];
            
            for path in safari_paths {
                if path.exists() {
                    if let Some(item) = self.create_browser_data_item(&path, "Safari", "Browser Data", RiskLevel::High) {
                        items.push(item);
                    }
                }
            }
        }
    }

    // QQ浏览器
    fn scan_qq_browser(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Local").join("Tencent").join("QQBrowser"),
            home_dir.join("AppData").join("Roaming").join("Tencent").join("QQBrowser"),
        ];
        
        for path in paths {
            if path.exists() {
                if let Some(item) = self.create_browser_data_item(&path, "QQBrowser", "QQ浏览器数据", RiskLevel::High) {
                    items.push(item);
                }
            }
        }
    }

    // 360浏览器
    fn scan_360_browser(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Local").join("360Chrome"),
            home_dir.join("AppData").join("Local").join("360se6"),
            home_dir.join("AppData").join("Roaming").join("360se6"),
        ];
        
        for path in paths {
            if path.exists() {
                let name = if path.to_string_lossy().contains("Chrome") {
                    "360安全浏览器"
                } else {
                    "360极速浏览器"
                };
                if let Some(item) = self.create_browser_data_item(&path, name, "360浏览器数据", RiskLevel::High) {
                    items.push(item);
                }
            }
        }
    }

    // 搜狗浏览器
    fn scan_sogou(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Roaming").join("SogouExplorer");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Sogou", "搜狗浏览器数据", RiskLevel::High) {
                items.push(item);
            }
        }
    }

    // UC浏览器
    fn scan_uc_browser(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("UCBrowser");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "UCBrowser", "UC浏览器数据", RiskLevel::High) {
                items.push(item);
            }
        }
    }

    // 傲游浏览器
    fn scan_maxthon(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Roaming").join("Maxthon5");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Maxthon", "傲游浏览器数据", RiskLevel::High) {
                items.push(item);
            }
        }
    }

    // 百分浏览器
    fn scan_cent(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("CentBrowser");
        if path.exists() {
            self.scan_chromium_browser(&path, "CentBrowser", items);
        }
    }

    // Opera
    fn scan_opera(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Roaming").join("Opera Software").join("Opera Stable"),
            home_dir.join("AppData").join("Local").join("Programs").join("Opera"),
        ];
        
        for path in paths {
            if path.exists() {
                self.scan_chromium_browser(&path, "Opera", items);
            }
        }
    }

    // Opera GX
    fn scan_opera_gx(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Roaming").join("Opera Software").join("Opera GX Stable");
        if path.exists() {
            self.scan_chromium_browser(&path, "Opera GX", items);
        }
    }

    // Brave
    fn scan_brave(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Local").join("BraveSoftware").join("Brave-Browser"),
            home_dir.join("AppData").join("Local").join("BraveSoftware").join("Brave-Browser-Nightly"),
        ];
        
        for path in paths {
            if path.exists() {
                self.scan_chromium_browser(&path, "Brave", items);
            }
        }
    }

    // Vivaldi
    fn scan_vivaldi(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("Vivaldi").join("User Data");
        if path.exists() {
            self.scan_chromium_browser(&path.parent().unwrap_or(&path), "Vivaldi", items);
        }
    }

    // Yandex
    fn scan_yandex(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("Yandex").join("YandexBrowser");
        if path.exists() {
            self.scan_chromium_browser(&path, "Yandex", items);
        }
    }

    // Tor Browser
    fn scan_tor(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Roaming").join("Tor"),
            home_dir.join("Desktop").join("Tor Browser"),
        ];
        
        for path in paths {
            if path.exists() {
                if let Some(item) = self.create_browser_data_item(&path, "Tor", "Tor浏览器数据", RiskLevel::Critical) {
                    items.push(item);
                }
            }
        }
    }

    // Waterfox
    fn scan_waterfox(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Roaming").join("Waterfox");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Waterfox", "Waterfox浏览器数据", RiskLevel::High) {
                items.push(item);
            }
        }
    }

    // Pale Moon
    fn scan_pale_moon(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Roaming").join("Moonchild Productions").join("Pale Moon");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "PaleMoon", "Pale Moon浏览器数据", RiskLevel::High) {
                items.push(item);
            }
        }
    }

    // SeaMonkey
    fn scan_seamonkey(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Roaming").join("Mozilla").join("SeaMonkey");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "SeaMonkey", "SeaMonkey浏览器数据", RiskLevel::High) {
                items.push(item);
            }
        }
    }

    // 雷神浏览器 (跨境电商)
    fn scan_thor(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("ThorBrowser");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Thor", "雷神浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // AdsPower (指纹浏览器)
    fn scan_adspower(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Local").join("AdsPower"),
            home_dir.join("AppData").join("Roaming").join("AdsPower"),
        ];
        
        for path in paths {
            if path.exists() {
                if let Some(item) = self.create_browser_data_item(&path, "AdsPower", "AdsPower指纹浏览器数据", RiskLevel::Critical) {
                    items.push(item);
                }
            }
        }
    }

    // Multilogin
    fn scan_multilogin(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let paths = vec![
            home_dir.join("AppData").join("Local").join("Multilogin"),
            home_dir.join("AppData").join("Roaming").join("Multilogin"),
        ];
        
        for path in paths {
            if path.exists() {
                if let Some(item) = self.create_browser_data_item(&path, "Multilogin", "Multilogin指纹浏览器数据", RiskLevel::Critical) {
                    items.push(item);
                }
            }
        }
    }

    // 拉力猫
    fn scan_lalicat(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("Lalicat");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Lalicat", "拉力猫指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // ixBrowser
    fn scan_ixbrowser(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("ixBrowser");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "ixBrowser", "ixBrowser指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // VMLogin
    fn scan_vmlogin(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("VMLogin");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "VMLogin", "VMLogin指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // HubStudio
    fn scan_hubstudio(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("HubStudio");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "HubStudio", "HubStudio指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // Dolphin
    fn scan_dolphin(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("Dolphin");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Dolphin", "Dolphin指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // MoreLogin
    fn scan_morelogin(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("MoreLogin");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "MoreLogin", "MoreLogin指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // GoLogin
    fn scan_gologin(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("GoLogin");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "GoLogin", "GoLogin指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // Incogniton
    fn scan_incogniton(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("Incogniton");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "Incogniton", "Incogniton指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // ClonBrowser
    fn scan_clonbrowser(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("ClonBrowser");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "ClonBrowser", "ClonBrowser指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // Octo Browser
    fn scan_octobrowser(&self, home_dir: &Path, items: &mut Vec<DataItem>) {
        let path = home_dir.join("AppData").join("Local").join("OctoBrowser");
        if path.exists() {
            if let Some(item) = self.create_browser_data_item(&path, "OctoBrowser", "OctoBrowser指纹浏览器数据", RiskLevel::Critical) {
                items.push(item);
            }
        }
    }

    // Helper methods
    fn get_browser_paths(&self, home_dir: &Path, company: &str, browser: &str, linux_name: &str) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        if cfg!(target_os = "windows") {
            paths.push(home_dir.join("AppData").join("Local").join(company).join(browser).join("User Data"));
        } else if cfg!(target_os = "macos") {
            paths.push(home_dir.join("Library").join("Application Support").join(company).join(browser));
        } else {
            paths.push(home_dir.join(".config").join(linux_name));
        }
        
        paths
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
            ("History", "历史记录", RiskLevel::High),
            ("Cookies", "Cookie数据", RiskLevel::High),
            ("Login Data", "登录凭据", RiskLevel::Critical),
            ("Bookmarks", "书签", RiskLevel::Medium),
            ("Preferences", "首选项配置", RiskLevel::Medium),
            ("Web Data", "自动填充数据", RiskLevel::High),
            ("Network Action Predictor", "网络预测数据", RiskLevel::Medium),
            ("Shortcuts", "快捷方式", RiskLevel::Low),
            ("Top Sites", "常用网站", RiskLevel::Medium),
            ("Visited Links", "访问链接", RiskLevel::Medium),
            ("Favicons", "网站图标", RiskLevel::Low),
            ("Current Session", "当前会话", RiskLevel::High),
            ("Current Tabs", "当前标签页", RiskLevel::High),
            ("Last Session", "上次会话", RiskLevel::High),
            ("Last Tabs", "上次标签页", RiskLevel::High),
        ];

        for (file, description, risk_level) in files {
            let file_path = profile_path.join(file);
            if file_path.exists() && file_path.is_file() {
                if let Some(item) = self.create_chromium_data_item(&file_path, browser_name, profile_name, description, risk_level) {
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
            ("places.sqlite", "历史记录和书签", RiskLevel::High),
            ("cookies.sqlite", "Cookie数据", RiskLevel::High),
            ("logins.json", "登录凭据", RiskLevel::Critical),
            ("formhistory.sqlite", "表单历史", RiskLevel::Medium),
            ("key4.db", "密码数据库", RiskLevel::Critical),
            ("cert9.db", "证书数据库", RiskLevel::Medium),
            ("permissions.sqlite", "权限设置", RiskLevel::Medium),
            ("webappsstore.sqlite", "Web应用存储", RiskLevel::Medium),
            ("chromeappsstore.sqlite", "Chrome应用存储", RiskLevel::Medium),
            ("sessionstore.jsonlz4", "会话存储", RiskLevel::High),
            ("sessionCheckpoints.json", "会话检查点", RiskLevel::High),
            ("prefs.js", "首选项配置", RiskLevel::Medium),
            ("handlers.json", "协议处理程序", RiskLevel::Low),
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

    fn create_chromium_data_item(&self, path: &Path, browser_name: &str, profile_name: &str, description: &str, risk_level: RiskLevel) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

            let file_name = path.file_name()?.to_string_lossy().to_string();
            let id = format!("browser-{}-{}-{}", browser_name.to_lowercase().replace(' ', "-"), profile_name.to_lowercase().replace(' ', "-"), file_name.to_lowercase());

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
            let id = format!("browser-firefox-{}-{}", profile_name.replace('.', "-"), file_name.to_lowercase());

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

    fn create_browser_data_item(&self, path: &Path, browser_name: &str, description: &str, risk_level: RiskLevel) -> Option<DataItem> {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let created_at = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let modified_at = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

            let id = format!("browser-{}-data", browser_name.to_lowercase().replace(' ', "-"));

            let mut item = DataItem::new(
                id,
                path.to_string_lossy().to_string(),
                DataType::Other,
                risk_level,
                size,
                created_at,
                modified_at,
            );

            item.description = Some(format!("{} - {}", browser_name, description));

            Some(item)
        } else {
            None
        }
    }
}
