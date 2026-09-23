// Tauri Imports
use tauri::async_runtime::Mutex;
use tauri::Manager;

use crate::managers::graph_manager::GraphManager;

pub mod api;
pub mod core;
pub mod dto;
pub mod managers;
pub mod nodes;
pub mod types;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            api::core::add_graph,
            api::core::get_graph_dto,
            api::core::get_node_types,
        ])
        .setup(|app| {
            app.manage(Mutex::new(GraphManager::new()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
