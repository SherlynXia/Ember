#![allow(dependency_on_unit_never_type_fallback)]   // 忽略单元类型不存在的警告

mod models;
mod storage;
mod services;
mod commands;

// 从重导出路径导入命令函数
use commands::{
    create_task, complete_task, get_tasks_by_date,
    save_daily_rating, get_daily_rating, get_rating_list, get_day_detail, delete_daily_rating,
    get_heatmap_data,
};
use models::AppState;
use std::sync::Mutex;
use storage::load_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载已有数据，若不存在则使用默认
    let app_state = load_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(app_state))
        .invoke_handler(tauri::generate_handler![
            create_task,
            complete_task,
            get_tasks_by_date,
            save_daily_rating,
            get_daily_rating,
            get_rating_list,
            get_day_detail,
            delete_daily_rating,
            get_heatmap_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}