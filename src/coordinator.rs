use crate::models::DataItem;
use crate::scanner::{Scanner, ScanResult, ScanError};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Coordinator {
    scanners: Vec<Arc<dyn Scanner>>,
    scan_cache: HashMap<String, Vec<DataItem>>,
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            scanners: Vec::new(),
            scan_cache: HashMap::new(),
        }
    }

    pub fn register_scanner(&mut self, scanner: Arc<dyn Scanner>) {
        self.scanners.push(scanner);
    }

    pub fn scan(&mut self, path: &str) -> ScanResult {
        if let Some(cached) = self.scan_cache.get(path) {
            return Ok(cached.clone());
        }

        let mut all_items = Vec::new();

        for scanner in &self.scanners {
            if scanner.supports_path(path) {
                match scanner.scan(path) {
                    Ok(items) => all_items.extend(items),
                    Err(e) => {
                        eprintln!("Scanner '{}' error: {}", scanner.name(), e);
                    }
                }
            }
        }

        self.scan_cache.insert(path.to_string(), all_items.clone());
        Ok(all_items)
    }

    pub fn clear_cache(&mut self) {
        self.scan_cache.clear();
    }

    pub fn invalidate_cache(&mut self, path: &str) {
        self.scan_cache.remove(path);
    }

    pub fn registered_scanners(&self) -> Vec<&str> {
        self.scanners.iter().map(|s| s.name()).collect()
    }
}

impl Default for Coordinator {
    fn default() -> Self {
        Self::new()
    }
}
