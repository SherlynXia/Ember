// 提供前端所需的热力图数据
use tauri::command;
use chrono::Duration;
use crate::models::AppState;
use serde::Serialize;

/// 成就趋势数据点
#[derive(Debug, Clone, Serialize)]
pub struct AchievementTrendPoint {
    pub date: String,
    pub achievement_score: u8,
    pub work_type: String,
}

#[command]
pub fn get_heatmap_data(
    state: tauri::State<'_, AppState>,
    start_date: String,
    end_date: String,
    data_type: String, // "deviation" 或 "rating"
) -> Result<serde_json::Value, String> {
    use chrono::NaiveDate;
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").map_err(|_| "起始日期错误")?;
    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").map_err(|_| "结束日期错误")?;

    let mut result = Vec::new();
    let mut current = start;
    while current <= end {
        let tasks: Vec<&crate::models::Task> = state.tasks.iter()
            .filter(|t| t.created_at == current)
            .collect();
        let row: Vec<f64> = tasks.iter()
            .map(|t| {
                if data_type == "deviation" {
                    // 计算偏差
                    if let (Some(actual), Some(est)) = (t.actual_time, Some(t.estimated_time)) {
                        (actual - est).abs() as f64
                    } else { 0.0 }
                } else {
                    // 评分或其他
                    t.energy_score.unwrap_or(0) as f64 / 5.0
                }
            })
            .collect();
        result.push(row);
        current = current + Duration::days(1);
    }

    Ok(serde_json::json!({ "data": result }))
}

/// 获取成就分数趋势数据（用于成长度趋势图）
#[command]
pub fn get_achievement_trend(
    state: tauri::State<'_, std::sync::Mutex<AppState>>,
    days: u32,
) -> Result<Vec<AchievementTrendPoint>, String> {
    use std::collections::BTreeMap;

    let state = state.lock().unwrap();
    let today = chrono::Local::now().naive_local().date();
    let start = today - Duration::days(days as i64);

    // 收集日期范围内的评分记录，按日期排序
    let mut sorted: BTreeMap<chrono::NaiveDate, &crate::models::DailyRating> = BTreeMap::new();
    for (date, rating) in &state.daily_ratings {
        if *date >= start && *date <= today {
            sorted.insert(*date, rating);
        }
    }

    let result: Vec<AchievementTrendPoint> = sorted
        .iter()
        .map(|(date, rating)| AchievementTrendPoint {
            date: date.format("%Y-%m-%d").to_string(),
            achievement_score: rating.achievement_score,
            work_type: rating.work_type.clone(),
        })
        .collect();

    Ok(result)
}