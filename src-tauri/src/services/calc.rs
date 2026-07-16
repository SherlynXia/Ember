// 偏差计算函数模块，用于计算任务偏差并返回相应的颜色值。

use crate::models::task::Task;

pub fn calculate_deviation(task: &Task) -> Option<f32> {
    if let (Some(actual), Some(est)) = (task.actual_time, Some(task.estimated_time)) {
        Some((actual - est).abs())
    } else {
        None
    }
}

pub fn deviation_color(deviation: f32, threshold: f32) -> &'static str {
    if deviation > threshold { "red" } else { "green" }
}