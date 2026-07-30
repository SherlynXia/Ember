use tauri::command;
use crate::models::{AppState, DailyRating};
use crate::storage::save_state;
use crate::services::{
    generate_color,
    calculate_achievement_score,
};
use std::sync::Mutex;
use chrono::{NaiveDate, Local};

// ==================== 命令函数 ====================

/// 保存每日评分
/// 前端调用：invoke("save_daily_rating", { date_str, work_type, energy_score, value_score, work_hours, mood_emoji, comment })
#[command]
pub fn save_daily_rating(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
    work_type: String,
    energy_score: u8,
    value_score: u8,
    work_hours: f32,
    mood_emoji: String,
    comment: Option<String>,
) -> Result<DailyRating, String> {
    // 参数校验
    if !["green", "blue", "orange", "gray"].contains(&work_type.as_str()) {
        return Err("工作状态类型必须是 green/blue/orange/gray 之一".to_string());
    }
    if !(1..=5).contains(&energy_score) {
        return Err("精力消耗分必须在1~5之间".to_string());
    }
    if !(1..=5).contains(&value_score) {
        return Err("贡献价值分必须在1~5之间".to_string());
    }
    if work_hours < 0.0 || work_hours > 24.0 {
        return Err("工作时长必须在0~24小时之间".to_string());
    }
    if !["😊", "😐", "😫", "🥳"].contains(&mood_emoji.as_str()) {
        return Err("心情表情必须是 😊/😐/😫/🥳 之一".to_string());
    }
    if let Some(ref c) = comment {
        if c.chars().count() > 50 {
            return Err("评语不能超过50个字".to_string());
        }
    }

    let mut state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    // 计算生成字段（使用共享计算模块）
    let productivity_color = generate_color(&work_type, energy_score, value_score);
    let achievement_score = calculate_achievement_score(energy_score, value_score);

    // 创建或更新评分记录
    let rating = DailyRating {
        date,
        work_type: work_type.clone(),
        energy_score,
        value_score,
        work_hours,
        mood_emoji: mood_emoji.clone(),
        comment: comment.clone(),
        productivity_color: productivity_color.clone(),
        achievement_score,
        status: 0,
        mit_completion_rate: 0.0,
        task_count_delta: 0,
        has_overdue: false,
        has_milestone: false,
    };

    state.daily_ratings.insert(date, rating.clone());
    save_state(&state)?;

    Ok(rating)
}

/// 获取指定日期的评分
#[command]
pub fn get_daily_rating(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
) -> Result<DailyRating, String> {
    let state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    match state.daily_ratings.get(&date) {
        Some(rating) => Ok(rating.clone()),
        None => Err("该日期暂无评分记录".to_string()),
        // None => Ok(DailyRating::new_empty(date)), // 已禁用：不再返回默认空记录
    }
}

/// 获取日期范围内的评分列表（用于热力图展示）
/// 只返回实际有记录的数据，今天及以后的未填写日期不返回
#[command]
pub fn get_rating_list(
    state: tauri::State<'_, Mutex<AppState>>,
    start_date: String,
    end_date: String,
) -> Result<Vec<DailyRating>, String> {
    let state = state.lock().unwrap();

    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
        .map_err(|_| "起始日期格式错误，请使用 YYYY-MM-DD".to_string())?;
    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
        .map_err(|_| "结束日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    // 获取今天的日期
    let today = Local::now().naive_local().date();

    let mut result = Vec::new();
    let mut current = start;

    while current <= end {
        // 只处理今天及之前的日期
        if current <= today {
            // 只返回实际有记录的数据，不返回 new_empty
            if let Some(rating) = state.daily_ratings.get(&current) {
                result.push(rating.clone());
            }
        }
        // 今天之后的日期直接跳过，不加入结果
        current = current + chrono::Duration::days(1);
    }

    Ok(result)
}

/// 获取某天完整详情（评分 + 任务列表）
#[command]
pub fn get_day_detail(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
) -> Result<serde_json::Value, String> {
    use serde_json::json;

    let state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    // 获取评分（不再返回默认空记录）
    let rating = match state.daily_ratings.get(&date) {
        Some(r) => r.clone(),
        None => return Err("该日期暂无评分记录".to_string()),
    };
    // let rating = state.daily_ratings.get(&date)
    //     .cloned()
    //     .unwrap_or_else(|| DailyRating::new_empty(date)); // 已禁用

    // 获取当天任务（创建或完成的）
    let tasks: Vec<&crate::models::Task> = state.tasks.iter()
        .filter(|t| t.created_at == date || t.completion_date == Some(date))
        .collect();

    Ok(json!({
        "rating": rating,
        "tasks": tasks
    }))
}

/// 删除指定日期的评分
#[command]
pub fn delete_daily_rating(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    state.daily_ratings.remove(&date);
    save_state(&state)?;
    Ok(())
}