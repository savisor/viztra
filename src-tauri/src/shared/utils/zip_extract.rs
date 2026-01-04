//! ZIP extraction utilities

use crate::shared::error::AppError;
use std::fs::File;
use std::path::PathBuf;
use zip::ZipArchive;

/// Extracts a ZIP file to a target directory
/// 
/// The ZIP file typically contains a folder named `repo-branch/`, so we extract
/// its contents to the target directory, skipping the root folder.
pub fn extract_zip(zip_path: &PathBuf, target_dir: &PathBuf) -> Result<(), AppError> {
    let file = File::open(zip_path)
        .map_err(|e| AppError::new(format!("Failed to open ZIP file: {}", e)))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| AppError::new(format!("Failed to read ZIP archive: {}", e)))?;

    // Ensure target directory exists
    std::fs::create_dir_all(target_dir)
        .map_err(|e| AppError::new(format!("Failed to create target directory: {}", e)))?;

    // Find the root folder name (first entry in the ZIP)
    let root_folder = if archive.len() > 0 {
        let first_file = archive.by_index(0)
            .map_err(|e| AppError::new(format!("Failed to read ZIP entry: {}", e)))?;
        let name = first_file.name();
        if let Some(slash_pos) = name.find('/') {
            name[..slash_pos].to_string()
        } else {
            String::new()
        }
    } else {
        return Err(AppError::new("ZIP archive is empty"));
    };

    // Extract all files, skipping the root folder
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| AppError::new(format!("Failed to read ZIP entry: {}", e)))?;

        let file_path = file.name();
        
        // Skip if it's the root folder itself
        if file_path == format!("{}/", root_folder) || file_path == root_folder {
            continue;
        }

        // Remove root folder prefix
        let relative_path = if file_path.starts_with(&format!("{}/", root_folder)) {
            &file_path[root_folder.len() + 1..]
        } else {
            file_path
        };

        let out_path = target_dir.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::new(format!("Failed to create directory: {}", e)))?;
        }

        // Extract file
        if file.is_dir() {
            std::fs::create_dir_all(&out_path)
                .map_err(|e| AppError::new(format!("Failed to create directory: {}", e)))?;
        } else {
            let mut out_file = File::create(&out_path)
                .map_err(|e| AppError::new(format!("Failed to create file: {}", e)))?;
            std::io::copy(&mut file, &mut out_file)
                .map_err(|e| AppError::new(format!("Failed to extract file: {}", e)))?;
        }
    }

    Ok(())
}
