use crate::models::DataItem;
use log::{info, warn, error};
use rand::Rng;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};
use std::path::Path;
use walkdir::WalkDir;
use thiserror::Error;
use chrono::Local;

#[derive(Error, Debug)]
pub enum CleanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Cleanup cancelled by user")]
    Cancelled,
    #[error("Other error: {0}")]
    Other(String),
}

pub type CleanResult = Result<(), CleanError>;

#[derive(Debug, Clone)]
pub enum CleanStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct CleanTask {
    pub item: DataItem,
    pub status: CleanStatus,
    pub start_time: Option<std::time::SystemTime>,
    pub end_time: Option<std::time::SystemTime>,
}

impl CleanTask {
    pub fn new(item: DataItem) -> Self {
        Self {
            item,
            status: CleanStatus::Pending,
            start_time: None,
            end_time: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CleanLog {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub item_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub struct Cleaner {
    tasks: Vec<CleanTask>,
    logs: Vec<CleanLog>,
    overwrite_passes: u8,
}

impl Cleaner {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            logs: Vec::new(),
            overwrite_passes: 3,
        }
    }

    pub fn with_overwrite_passes(mut self, passes: u8) -> Self {
        self.overwrite_passes = passes.max(1);
        self
    }

    pub fn add_task(&mut self, item: DataItem) {
        self.tasks.push(CleanTask::new(item));
    }

    pub fn add_tasks(&mut self, items: Vec<DataItem>) {
        for item in items {
            self.tasks.push(CleanTask::new(item));
        }
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn tasks(&self) -> &[CleanTask] {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut [CleanTask] {
        &mut self.tasks
    }

    pub fn logs(&self) -> &[CleanLog] {
        &self.logs
    }

    pub fn clean_all<F>(&mut self, mut confirm_callback: F) -> CleanResult
    where
        F: FnMut(&[CleanTask]) -> bool,
    {
        if self.tasks.is_empty() {
            self.log(LogLevel::Warning, "No tasks to clean".to_string(), None);
            return Ok(());
        }

        if !confirm_callback(&self.tasks) {
            self.log(LogLevel::Info, "Cleanup cancelled by user".to_string(), None);
            return Err(CleanError::Cancelled);
        }

        let mut success_count = 0;
        let mut fail_count = 0;

        for task in &mut self.tasks {
            task.status = CleanStatus::InProgress;
            task.start_time = Some(std::time::SystemTime::now());

            match self.clean_item(&task.item) {
                Ok(_) => {
                    task.status = CleanStatus::Completed;
                    success_count += 1;
                    self.log(
                        LogLevel::Info,
                        format!("Successfully cleaned: {}", task.item.path),
                        Some(task.item.path.clone()),
                    );
                }
                Err(e) => {
                    task.status = CleanStatus::Failed(e.to_string());
                    fail_count += 1;
                    self.log(
                        LogLevel::Error,
                        format!("Failed to clean {}: {}", task.item.path, e),
                        Some(task.item.path.clone()),
                    );
                }
            }

            task.end_time = Some(std::time::SystemTime::now());
        }

        self.log(
            LogLevel::Info,
            format!("Cleanup completed: {} succeeded, {} failed", success_count, fail_count),
            None,
        );

        Ok(())
    }

    fn clean_item(&self, item: &DataItem) -> CleanResult {
        let path = Path::new(&item.path);

        if !path.exists() {
            return Err(CleanError::FileNotFound(item.path.clone()));
        }

        if path.is_dir() {
            self.clean_directory(path)?;
        } else {
            self.clean_file(path)?;
        }

        Ok(())
    }

    fn clean_file(&self, path: &Path) -> CleanResult {
        info!("Cleaning file: {}", path.display());

        if let Ok(metadata) = fs::metadata(path) {
            let file_size = metadata.len();
            if file_size > 0 {
                for pass in 1..=self.overwrite_passes {
                    self.overwrite_file(path, pass, self.overwrite_passes)?;
                }
            }
        }

        fs::remove_file(path)?;
        info!("File removed: {}", path.display());

        Ok(())
    }

    fn overwrite_file(&self, path: &Path, pass: u8, total_passes: u8) -> CleanResult {
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)?;

        let file_size = file.metadata()?.len();
        if file_size == 0 {
            return Ok(());
        }

        let mut rng = rand::thread_rng();
        let buffer_size = 4096;
        let mut buffer = vec![0u8; buffer_size];

        file.seek(SeekFrom::Start(0))?;

        let mut bytes_written = 0;
        while bytes_written < file_size {
            let write_size = std::cmp::min(buffer_size, (file_size - bytes_written) as usize);

            for byte in &mut buffer[..write_size] {
                *byte = match pass % 3 {
                    1 => 0x00,
                    2 => 0xFF,
                    _ => rng.gen(),
                };
            }

            file.write_all(&buffer[..write_size])?;
            bytes_written += write_size as u64;
        }

        file.sync_all()?;
        info!("Overwrite pass {}/{} completed for {}", pass, total_passes, path.display());

        Ok(())
    }

    fn clean_directory(&self, path: &Path) -> CleanResult {
        info!("Cleaning directory: {}", path.display());

        for entry in WalkDir::new(path).contents_first(true) {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                self.clean_file(entry_path)?;
            } else if entry_path.is_dir() && entry_path != path {
                fs::remove_dir(entry_path)?;
                info!("Directory removed: {}", entry_path.display());
            }
        }

        fs::remove_dir(path)?;
        info!("Directory removed: {}", path.display());

        Ok(())
    }

    fn log(&mut self, level: LogLevel, message: String, item_path: Option<String>) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match level {
            LogLevel::Info => info!("{}", message),
            LogLevel::Warning => warn!("{}", message),
            LogLevel::Error => error!("{}", message),
        }

        self.logs.push(CleanLog {
            timestamp,
            level,
            message,
            item_path,
        });
    }

    pub fn export_logs(&self, output_path: &str) -> CleanResult {
        let mut file = File::create(output_path)?;

        writeln!(file, "Cleanup Log - Generated: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file, "=")?;
        writeln!(file)?;

        for log in &self.logs {
            let level_str = match log.level {
                LogLevel::Info => "[INFO]",
                LogLevel::Warning => "[WARN]",
                LogLevel::Error => "[ERROR]",
            };

            if let Some(path) = &log.item_path {
                writeln!(file, "{} {} - {} (Path: {})", log.timestamp, level_str, log.message, path)?;
            } else {
                writeln!(file, "{} {} - {}", log.timestamp, level_str, log.message)?;
            }
        }

        Ok(())
    }
}

impl Default for Cleaner {
    fn default() -> Self {
        Self::new()
    }
}
