use tauri::command;
use crate::models::{Task, AppState};
use crate::storage::{load_state, save_state, get_state_path};
use crate::services::calculate_deviation;
use chrono::Local;
use std::sync::Mutex;

#[command]
pub fn create_task(
    state: tauri::State<'_, Mutex<AppState>>,
    title: String,
    estimated_time: f32,
    estimated_difficulty: u8,
    is_important: bool,
) -> Result<Task, String> {
    // 1. 参数校验
    if title.is_empty() {
        return Err("任务标题不能为空".to_string());
    }
    if !(1..=5).contains(&estimated_difficulty) {
        return Err("预估难度必须在1~5之间".to_string());
    }

    // 2. 构建新任务
    let mut state = state.lock().unwrap(); // 注意：AppState 需要用 Mutex 包裹，后面会改
    let id = state.next_task_id;
    state.next_task_id += 1;

    let task = Task {
        id,
        title,
        estimated_time,
        estimated_difficulty,
        actual_time: None,
        actual_satisfaction: None,
        is_completed: false,
        completion_date: None,
        is_important,
        emoji_mood: None,
        energy_score: None,
        value_score: None,
        comment: None,
        created_at: Local::now().date_naive(),
    };

    state.tasks.push(task.clone());

    // 3. 保存到文件
    save_state(&state)?;

    Ok(task)
}

#[command]
pub fn complete_task(
    state: tauri::State<'_, Mutex<AppState>>,
    task_id: u64,
    actual_time: f32,
    actual_satisfaction: u8,
    emoji_mood: String,
    energy_score: u8,
    value_score: u8,
    comment: Option<String>,
) -> Result<(), String> {
    // 参数校验
    if !(1..=5).contains(&actual_satisfaction) {
        return Err("满意度必须在1~5之间".to_string());
    }
    // ... 其他校验

    let mut state = state.lock().unwrap();
    let task = state.tasks.iter_mut().find(|t| t.id == task_id)
        .ok_or("任务不存在")?;

    if task.is_completed {
        return Err("任务已经完成".to_string());
    }

    // 更新字段
    task.actual_time = Some(actual_time);
    task.actual_satisfaction = Some(actual_satisfaction);
    task.is_completed = true;
    task.completion_date = Some(Local::now().date_naive());
    task.emoji_mood = Some(emoji_mood);
    task.energy_score = Some(energy_score);
    task.value_score = Some(value_score);
    task.comment = comment;

    // 触发每日评分更新（调用 services 里的函数）
    // 这里需要先实现 services/stats.rs 和 calc.rs 的相关函数
    // 比如：update_daily_rating(&mut state, task.completion_date.unwrap());

    save_state(&state)?;
    Ok(())
}

#[command]
pub fn get_tasks_by_date(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
) -> Result<Vec<Task>, String> {
    use chrono::NaiveDate;
    
    let state = state.lock().unwrap(); 
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    let tasks = state.tasks.iter()
        .filter(|t| t.created_at == date || t.completion_date == Some(date))
        .cloned()
        .collect();

    Ok(tasks)
}