use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRating {
    pub date: NaiveDate,

    // === 用户直接填写的核心字段 ===
    pub work_type: String,          // 工作状态类型: "green"(高产)/"blue"(学习)/"orange"(低效)/"gray"(休息)
    pub energy_score: u8,           // 精力消耗分 1-5（纵轴）
    pub value_score: u8,            // 贡献价值分 1-5（横轴）
    pub work_hours: f32,            // 有效专注时长（小时）
    pub mood_emoji: String,         // 心情表情: "😊"(爽)/"😐"(一般)/"😫"(累)/"🥳"(超常)
    pub comment: Option<String>,    // 每日评语（可选，≤50字）

    // === 后端计算生成的字段 ===
    pub productivity_color: String, // 最终RGBA颜色字符串，前端直接用于热力图显示
    pub achievement_score: u8,      // 综合成就分 0-100（根据value/energy计算）

    // === 扩展字段（可选）===
    pub status: u8,                 // 0-5 兼容旧版，暂时保留
    pub mit_completion_rate: f32,   // MIT任务完成率（从任务自动统计）
    pub task_count_delta: i8,       // 与昨日任务量差值
    pub has_overdue: bool,          // 是否有逾期任务
    pub has_milestone: bool,        // 是否达成里程碑
}

impl DailyRating {
    /// 创建一个新的空评分（用于默认值）
    pub fn new_empty(date: NaiveDate) -> Self {
        Self {
            date,
            work_type: "gray".to_string(),
            energy_score: 0,
            value_score: 0,
            work_hours: 0.0,
            mood_emoji: "⬜".to_string(),
            comment: None,
            productivity_color: "rgba(136, 136, 136, 0.3)".to_string(),
            achievement_score: 0,
            status: 0,
            mit_completion_rate: 0.0,
            task_count_delta: 0,
            has_overdue: false,
            has_milestone: false,
        }
    }
}