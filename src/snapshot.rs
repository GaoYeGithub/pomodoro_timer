use std::{
    collections::HashMap,
    path::Path,
};
use chrono::Local;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub size: u64,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectorySnapshot {
    pub timestamp: String,
    pub files: HashMap<String, FileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
    pub initial_state: Option<FileInfo>,
    pub final_state: Option<FileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeReport {
    pub session_start: String,
    pub session_end: String,
    pub changes: Vec<FileChange>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}

impl DirectorySnapshot {
    pub fn new(dir: &Path) -> std::io::Result<Self> {
        let mut files = HashMap::new();
        
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let metadata = entry.metadata()?;
                let relative_path = entry.path().strip_prefix(dir).unwrap().to_string_lossy().into_owned();
                
                files.insert(
                    relative_path,
                    FileInfo {
                        size: metadata.len(),
                        modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)
                            .unwrap().as_secs().to_string(),
                    },
                );
            }
        }

        Ok(DirectorySnapshot {
            timestamp: Local::now().to_rfc3339(),
            files,
        })
    }

    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn compare(&self, other: &DirectorySnapshot) -> Vec<FileChange> {
        let mut changes = Vec::new();

        for (path, final_info) in &self.files {
            match other.files.get(path) {
                Some(initial_info) => {
                    if initial_info.size != final_info.size || initial_info.modified != final_info.modified {
                        changes.push(FileChange {
                            path: path.clone(),
                            change_type: ChangeType::Modified,
                            initial_state: Some(initial_info.clone()),
                            final_state: Some(final_info.clone()),
                        });
                    }
                }
                None => {
                    changes.push(FileChange {
                        path: path.clone(),
                        change_type: ChangeType::Added,
                        initial_state: None,
                        final_state: Some(final_info.clone()),
                    });
                }
            }
        }

        for (path, initial_info) in &other.files {
            if !self.files.contains_key(path) {
                changes.push(FileChange {
                    path: path.clone(),
                    change_type: ChangeType::Deleted,
                    initial_state: Some(initial_info.clone()),
                    final_state: None,
                });
            }
        }

        changes
    }
}