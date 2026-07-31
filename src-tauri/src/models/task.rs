use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

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
    
    // === 新增字段 ===
    pub parent_id: Option<u64>,     // 父任务ID（子任务用）
    pub assigned_date: Option<NaiveDate>, // 分配到的日期（拖拽用）
    pub sub_task_ids: Vec<u64>,     // 子任务ID列表
    pub order_index: u32,           // 排序索引（用于拖拽排序）
}

impl Task {
    /// 检查是否是子任务
    pub fn is_sub_task(&self) -> bool {
        self.parent_id.is_some()
    }
    
    /// 检查是否有子任务
    pub fn has_sub_tasks(&self) -> bool {
        !self.sub_task_ids.is_empty()
    }
    
    /// 获取任务状态描述
    pub fn get_status_label(&self) -> &'static str {
        if self.is_completed {
            "已完成"
        } else if self.assigned_date.is_some() {
            "已安排"
        } else {
            "待安排"
        }
    }
}