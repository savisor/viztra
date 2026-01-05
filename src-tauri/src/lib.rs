mod features;
mod shared;

use features::{
    greet,
    pull_assets,
    pull_asset_by_symbol,
    list_symbols,
    retrieve_asset_ochl,
    validate_and_store_deals,
    read_deals_from_file,
    read_all_deals,
    execute_insight,
    execute_batch_insights,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the insight registry before starting Tauri
    features::insights::factory::initialize_registry();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            pull_assets,
            pull_asset_by_symbol,
            list_symbols,
            retrieve_asset_ochl,
            validate_and_store_deals,
            read_deals_from_file,
            read_all_deals,
            execute_insight,
            execute_batch_insights
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
