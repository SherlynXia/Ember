mod models;
mod storage;
mod services;
mod commands;

use commands::task_commands::{create_task, complete_task};
use commands::stats_commands::get_heatmap_data;
use storage::file_store;
use models::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载持久化数据
    let app_state = file_store::load_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state) // 注入状态
        .invoke_handler(tauri::generate_handler![
            create_task,
            complete_task,
            get_heatmap_data,
            // ... 其他命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
