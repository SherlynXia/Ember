use tauri::command;
use crate::models::{AppState, DailyRating};
use crate::storage::save_state;
use std::sync::Mutex;
use chrono::NaiveDate;

// ==================== 颜色计算函数 ====================

/// 根据工作状态类型获取基础色相（RGB）
fn work_type_to_rgb(work_type: &str) -> (u8, u8, u8) {
    match work_type {
        "green" => (34, 197, 94),    // 绿色系 - 高产日
        "blue" => (59, 130, 246),    // 蓝色系 - 学习日
        "orange" => (251, 146, 60),  // 橙色系 - 忙碌但低效日
        "gray" => (136, 136, 136),   // 灰色系 - 休息/摸鱼日
        _ => (136, 136, 136),        // 默认灰色
    }
}

/// 计算饱和度（alpha值）
/// 高价值 + 低消耗 = 高饱和度（深色，接近1.0）
/// 低价值 + 高消耗 = 低饱和度（浅色，接近0.3）
fn calculate_saturation(energy_score: u8, value_score: u8) -> f32 {
    if energy_score == 0 || value_score == 0 {
        return 0.3; // 默认低饱和度
    }

    // 效率指数 = 价值 / 消耗
    // energy_score: 1-5 (1=低消耗，5=高消耗)
    // value_score: 1-5 (1=低价值，5=高价值)
    // 高价值+低消耗 = 高效率
    let efficiency = value_score as f32 / energy_score as f32;

    // 将效率指数映射到饱和度范围 0.3 - 1.0
    // efficiency 范围大约是 0.2 (1/5) 到 5.0 (5/1)
    // 我们希望 efficiency=1.0 时饱和度适中，efficiency>1 时饱和度高
    let saturation = 0.3 + (efficiency.min(2.0) / 2.0) * 0.7;

    saturation.min(1.0).max(0.3)
}

/// 根据工作状态和二维评分生成最终RGBA颜色
fn generate_color(work_type: &str, energy_score: u8, value_score: u8) -> String {
    let (r, g, b) = work_type_to_rgb(work_type);
    let alpha = calculate_saturation(energy_score, value_score);
    format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha)
}

/// 计算成就分数（0-100）
/// 基于效率指数和价值分
fn calculate_achievement_score(energy_score: u8, value_score: u8) -> u8 {
    if energy_score == 0 || value_score == 0 {
        return 0;
    }
    // 效率指数 * 价值权重
    let efficiency = value_score as f32 / energy_score as f32;
    let score = (efficiency * value_score as f32 * 4.0).min(100.0) as u8;
    score
}

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

    // 计算生成字段
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
        None => Ok(DailyRating::new_empty(date)),
    }
}

/// 获取日期范围内的评分列表（用于热力图展示）
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

    let mut result = Vec::new();
    let mut current = start;

    while current <= end {
        let rating = state.daily_ratings.get(&current)
            .cloned()
            .unwrap_or_else(|| DailyRating::new_empty(current));
        result.push(rating);
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

    // 获取评分
    let rating = state.daily_ratings.get(&date)
        .cloned()
        .unwrap_or_else(|| DailyRating::new_empty(date));

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