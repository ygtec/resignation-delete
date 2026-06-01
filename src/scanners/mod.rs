pub mod git_ssh;
pub mod browsers;
pub mod jetbrains;
pub mod vscode;
pub mod ai_tools;

use crate::models::DataItem;

pub trait Scanner {
    fn name(&self) -> &str;
    fn scan(&self) -> Vec<DataItem>;
}
