use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub estimated_time: f32,        // 预估耗时（小时）
    pub estimated_difficulty: u8,   // 1~5
    pub actual_time: Option<f32>,   // 实际耗时
    pub actual_satisfaction: Option<u8>, // 1~5
    pub is_completed: bool,
    pub completion_date: Option<NaiveDate>,
    pub is_important: bool,         // 是否为 MIT（最重要任务）
    pub emoji_mood: Option<String>, // 😊😐😫🥳
    pub energy_score: Option<u8>,   // 1~5
    pub value_score: Option<u8>,    // 1~5
    pub comment: Option<String>,    // 碎片化批注（≤20字）
    pub created_at: NaiveDate,
}