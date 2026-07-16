use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRating {
    pub date: NaiveDate,
    pub productivity_color: String, // "green", "blue", "orange", "gray"
    pub work_hours: f32,            // 有效专注时长
    pub mood_emoji: String,         // 😊😐😫🥳
    pub achievement_score: u8,      // 0~100，中央大字
    pub mit_completion_rate: f32,   // 最重要任务完成率
    pub task_count_delta: i8,       // 与昨日任务量差值（+2 / -1）
    pub has_overdue: bool,          // 左上角红点
    pub has_milestone: bool,        // 右上角星星/奖杯
    pub comments: Vec<String>,      // 多条短评
}