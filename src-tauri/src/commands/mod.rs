// 声明子模块
mod task_commands;
mod rating_commands;
mod stats_commands;

// 把命令函数导出，方便在 lib.rs 中注册
pub use task_commands::{create_task, complete_task, get_tasks_by_date};
pub use rating_commands::{
    save_daily_rating,           // 保存每日评分（完整版）
    get_daily_rating,            // 获取单日评分
    get_rating_list,             // 获取日期范围评分列表
    get_day_detail,              // 获取某天完整详情（评分+任务）
    delete_daily_rating,         // 删除评分
};
pub use stats_commands::{get_heatmap_data, get_achievement_trend};