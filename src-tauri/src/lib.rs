#![allow(dependency_on_unit_never_type_fallback)]   // 忽略单元类型不存在的警告

mod models;
mod storage;
mod services;
mod commands;

// 从重导出路径导入命令函数
use commands::{
    // 任务命令
    create_task, update_task, delete_task, complete_task,
    get_tasks_by_date, get_unassigned_tasks, get_all_tasks, get_task_detail,
    assign_task_to_date, reorder_tasks,
    create_sub_task, get_sub_tasks, promote_sub_task,
    // 评分命令
    save_daily_rating, get_daily_rating, get_rating_list, get_day_detail, delete_daily_rating,
    // 统计命令
    get_heatmap_data, get_achievement_trend,
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
            // 任务基础 CRUD
            create_task,
            update_task,
            delete_task,
            complete_task,
            // 任务查询
            get_tasks_by_date,
            get_unassigned_tasks,
            get_all_tasks,
            get_task_detail,
            // 任务拖拽和日期分配
            assign_task_to_date,
            reorder_tasks,
            // 子任务
            create_sub_task,
            get_sub_tasks,
            promote_sub_task,
            // 评分
            save_daily_rating,
            get_daily_rating,
            get_rating_list,
            get_day_detail,
            delete_daily_rating,
            // 统计
            get_heatmap_data,
            get_achievement_trend,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}