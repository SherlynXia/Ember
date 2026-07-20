// commands/rating_commands.rs
use tauri::command;
use crate::models::AppState;
use chrono::{NaiveDate, Duration};

#[command]
pub fn get_daily_rating(
    state: tauri::State<'_, AppState>,
    date_str: String,
) -> Result<crate::models::DailyRating, String> {
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误")?;
    state.daily_ratings.get(&date)
        .cloned()
        .ok_or_else(|| "当天无评分数据".to_string())
}
