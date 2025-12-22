use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileSystemEvent {
    pub action: String,
    pub path: String,
    pub is_directory: bool,
    pub timestamp: String,
}

