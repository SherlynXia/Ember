mod models;
mod storage;
mod services;
mod commands;

// 从重导出路径导入命令函数
use commands::{create_task, complete_task, get_tasks_by_date, get_daily_rating, get_heatmap_data};
use storage::load_state;  // 用于加载数据
use models::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载已有数据，若不存在则使用默认
    let app_state = load_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(app_state))
        .invoke_handler(tauri::generate_handler![
            create_task,
            complete_task,
            get_tasks_by_date,
            get_daily_rating,
            get_heatmap_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
