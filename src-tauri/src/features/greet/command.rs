//! Greet command handlers

use crate::features::greet::service::GreetService;

/// Tauri command: greet
/// Returns a greeting message
#[tauri::command]
pub fn greet(name: String) -> Result<String, String> {
    GreetService::create_greeting(&name).map_err(|e| e.message)
}
