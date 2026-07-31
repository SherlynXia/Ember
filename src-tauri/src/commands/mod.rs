// 声明子模块
mod task_commands;
mod rating_commands;
mod stats_commands;
mod pet_commands;
mod audio_commands;

// 把命令函数导出，方便在 lib.rs 中注册
pub use task_commands::{
    // 基础 CRUD
    create_task,
    update_task,
    delete_task,
    complete_task,
    // 查询
    get_tasks_by_date,
    get_unassigned_tasks,
    get_all_tasks,
    get_task_detail,
    // 拖拽和日期分配
    assign_task_to_date,
    reorder_tasks,
    // 子任务
    create_sub_task,
    get_sub_tasks,
    promote_sub_task,
};
pub use rating_commands::{
    save_daily_rating,           // 保存每日评分（完整版）
    get_daily_rating,            // 获取单日评分
    get_rating_list,             // 获取日期范围评分列表
    get_day_detail,              // 获取某天完整详情（评分+任务）
    delete_daily_rating,         // 删除评分
};
pub use stats_commands::{get_heatmap_data, get_achievement_trend};
pub use pet_commands::{get_screen_size, get_pet_window_position, move_pet_window};
pub use audio_commands::{play_audio, play_random_startup_audio, play_random_time_audio};