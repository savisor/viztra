//! File cleanup service for removing unwanted files and directories

use crate::shared::error::AppError;
use std::path::{Path, PathBuf};

/// Service for cleaning up files and directories
pub struct FileCleanupService;

impl FileCleanupService {
    /// Removes all .md files recursively from a directory
    pub fn remove_markdown_files(dir: &PathBuf) -> Result<(), AppError> {
        Self::remove_files_by_extension(dir, ".md")?;
        Self::remove_files_by_extension(dir, ".MD")?;
        Ok(())
    }

    /// Removes .git directory if it exists
    pub fn remove_git_directory(dir: &PathBuf) -> Result<(), AppError> {
        let git_dir = dir.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            std::fs::remove_dir_all(&git_dir)
                .map_err(|e| AppError::new(format!("Failed to remove .git directory: {}", e)))?;
        }
        Ok(())
    }

    /// Removes all files with a specific extension recursively
    fn remove_files_by_extension(dir: &Path, extension: &str) -> Result<(), AppError> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)
            .map_err(|e| AppError::new(format!("Failed to read directory: {}", e)))?
        {
            let entry = entry
                .map_err(|e| AppError::new(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                Self::remove_files_by_extension(&path, extension)?;
            } else if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.ends_with(extension) {
                        std::fs::remove_file(&path)
                            .map_err(|e| AppError::new(format!("Failed to remove file: {}", e)))?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Removes unwanted files and directories (.md, .git) from a directory
    pub fn cleanup_directory(dir: &PathBuf) -> Result<(), AppError> {
        Self::remove_markdown_files(dir)?;
        Self::remove_git_directory(dir)?;
        Ok(())
    }
}
