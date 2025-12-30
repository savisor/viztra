/**
 * Greet command handlers
 */

use crate::services::greet::GreetService;

/// Tauri command: greet
/// Returns a greeting message
#[tauri::command]
pub fn greet(name: String) -> Result<String, String> {
    GreetService::create_greeting(&name).map_err(|e| e.message)
}

