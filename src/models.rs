#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Document,
    Image,
    Video,
    Audio,
    Code,
    Archive,
    Other,
}

#[derive(Debug, Clone)]
pub struct DataItem {
    pub id: String,
    pub path: String,
    pub data_type: DataType,
    pub risk_level: RiskLevel,
    pub size: u64,
    pub created_at: std::time::SystemTime,
    pub modified_at: std::time::SystemTime,
    pub description: Option<String>,
}

impl DataItem {
    pub fn new(
        id: String,
        path: String,
        data_type: DataType,
        risk_level: RiskLevel,
        size: u64,
        created_at: std::time::SystemTime,
        modified_at: std::time::SystemTime,
    ) -> Self {
        Self {
            id,
            path,
            data_type,
            risk_level,
            size,
            created_at,
            modified_at,
            description: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
