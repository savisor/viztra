// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod models;
mod services;
mod utils;

use commands::{asset, greet};

fn main() {
    // Disable DMA-BUF renderer to fix blank screen issues on Linux
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    // Note: A webkit deprecation warning may appear in the console:
    // "webkit_settings_set_enable_offline_web_application_cache is deprecated"
    // This is a known issue from the Tauri/WebKit library itself and can be safely ignored.
    // It does not affect application functionality.

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet::greet,
            asset::pull_assets,
            asset::pull_asset_by_symbol,
            asset::list_symbols,
            asset::retrieve_asset_ochl
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
