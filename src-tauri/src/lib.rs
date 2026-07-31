#![allow(dependency_on_unit_never_type_fallback)]

mod models;
mod storage;
mod services;
mod commands;

use commands::{
    create_task, update_task, delete_task, complete_task,
    get_tasks_by_date, get_unassigned_tasks, get_all_tasks, get_task_detail,
    assign_task_to_date, reorder_tasks,
    create_sub_task, get_sub_tasks, promote_sub_task,
    save_daily_rating, get_daily_rating, get_rating_list, get_day_detail, delete_daily_rating,
    get_heatmap_data, get_achievement_trend,
    get_screen_size, get_pet_window_position, move_pet_window,
    play_audio, play_random_startup_audio, play_random_time_audio,
};
use models::AppState;
use std::sync::Mutex;
use storage::load_state;
use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = load_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(app_state))
        .setup(|app| {
            // 获取宠物窗口并显示
            if let Some(pet_window) = app.get_webview_window("pet") {
                // 设置宠物窗口在屏幕右下角
                if let Ok(monitor) = pet_window.current_monitor() {
                    if let Some(monitor) = monitor {
                        let size = monitor.size();
                        let scale_factor = monitor.scale_factor();
                        let window_x = (size.width as f64 / scale_factor - 270.0) as i32;
                        let window_y = (size.height as f64 / scale_factor - 270.0) as i32;
                        let _ = pet_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(window_x, window_y)));
                    }
                }
                // 设置透明效果
                #[cfg(windows)]
                {
                    use tauri::window::Color;
                    let _ = pet_window.set_background_color(Some(Color(0, 0, 0, 0)));
                }
                let _ = pet_window.show();
                let _ = pet_window.set_always_on_top(true);
                // 确保宠物窗口不抢夺焦点
                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.set_focus();
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            // 主窗口关闭时，宠物窗口也关闭
            if window.label() == "main" {
                if let WindowEvent::CloseRequested { .. } = event {
                    if let Some(pet_window) = window.app_handle().get_webview_window("pet") {
                        let _ = pet_window.close();
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            create_task,
            update_task,
            delete_task,
            complete_task,
            get_tasks_by_date,
            get_unassigned_tasks,
            get_all_tasks,
            get_task_detail,
            assign_task_to_date,
            reorder_tasks,
            create_sub_task,
            get_sub_tasks,
            promote_sub_task,
            save_daily_rating,
            get_daily_rating,
            get_rating_list,
            get_day_detail,
            delete_daily_rating,
            get_heatmap_data,
            get_achievement_trend,
            get_screen_size,
            get_pet_window_position,
            move_pet_window,
            play_audio,
            play_random_startup_audio,
            play_random_time_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}