use tauri::command;
use crate::models::{AppState, DailyRating};
use crate::storage::save_state;
use std::sync::Mutex;
use chrono::NaiveDate;

// 1. 返回 (r, g, b) 三元组
fn status_to_rgb(status: u8) -> (u8, u8, u8) {
    match status {
        0 => (136, 136, 136), // 灰色
        1 => (239, 68, 68),   // 红
        2 => (251, 146, 60),  // 橙
        3 => (250, 204, 21),  // 黄
        4 => (74, 222, 128),  // 浅绿
        5 => (34, 197, 94),   // 深绿
        _ => (136, 136, 136),
    }
}

// 2. 生成 rgba 字符串，透明度固定 0.85
fn status_to_rgba(status: u8) -> String {
    let (r, g, b) = status_to_rgb(status);
    format!("rgba({}, {}, {}, 0.85)", r, g, b)
}

// 3. 状态对应的表情
fn status_to_emoji(status: u8) -> &'static str {
    match status {
        0 => "⬜",
        1 => "❌",
        2 => "⚠️",
        3 => "🟡",
        4 => "🟢",
        5 => "✅",
        _ => "⬜",
    }
}

// 4. 保存命令（覆盖式评语）
#[command]
pub fn save_daily_rating(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
    status: u8,
    comment: Option<String>,
) -> Result<(), String> {
    if status > 5 {
        return Err("状态值必须在0~5之间".to_string());
    }
    if let Some(ref c) = comment {
        if c.chars().count() > 20 {
            return Err("评语不能超过20个字".to_string());
        }
    }

    let mut state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    let color = status_to_rgba(status);
    let emoji = status_to_emoji(status);

    let entry = state.daily_ratings.entry(date).or_insert_with(|| {
        DailyRating {
            date,
            status: 0,
            comment: None,
            mood_emoji: "⬜".to_string(),
            productivity_color: "rgba(136, 136, 136, 0.85)".to_string(),
            work_hours: 0.0,
            achievement_score: 0,
            mit_completion_rate: 0.0,
            task_count_delta: 0,
            has_overdue: false,
            has_milestone: false,
        }
    });

    // 覆盖更新
    entry.status = status;
    entry.comment = comment;
    entry.mood_emoji = emoji.to_string();
    entry.productivity_color = color;

    save_state(&state)?;
    Ok(())
}

// 5. 查询命令
#[command]
pub fn get_daily_rating(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
) -> Result<DailyRating, String> {
    let state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误")?;

    match state.daily_ratings.get(&date) {
        Some(rating) => Ok(rating.clone()),
        None => Ok(DailyRating {
            date,
            status: 0,
            comment: None,
            mood_emoji: "⬜".to_string(),
            productivity_color: "rgba(136, 136, 136, 0.85)".to_string(),
            work_hours: 0.0,
            achievement_score: 0,
            mit_completion_rate: 0.0,
            task_count_delta: 0,
            has_overdue: false,
            has_milestone: false,
        }),
    }
}