use tauri::command;
use crate::models::{Task, AppState};
use crate::storage::save_state;
use chrono::{Local, NaiveDate};
use std::sync::Mutex;

// ==================== 基础 CRUD ====================

/// 创建任务
#[command]
pub fn create_task(
    state: tauri::State<'_, Mutex<AppState>>,
    title: String,
    estimated_time: f32,
    estimated_difficulty: u8,
    is_important: bool,
) -> Result<Task, String> {
    // 参数校验
    if title.is_empty() {
        return Err("任务标题不能为空".to_string());
    }
    if !(1..=5).contains(&estimated_difficulty) {
        return Err("预估难度必须在1~5之间".to_string());
    }

    let mut state = state.lock().unwrap();
    let id = state.next_task_id;
    state.next_task_id += 1;

    // 计算排序索引（放在最后）
    let order_index = state.tasks.len() as u32;

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
        parent_id: None,
        assigned_date: None,
        sub_task_ids: Vec::new(),
        order_index,
    };

    state.tasks.push(task.clone());
    save_state(&state)?;

    Ok(task)
}

/// 更新任务（编辑）
#[command]
pub fn update_task(
    state: tauri::State<'_, Mutex<AppState>>,
    task_id: u64,
    title: Option<String>,
    estimated_time: Option<f32>,
    estimated_difficulty: Option<u8>,
    is_important: Option<bool>,
    comment: Option<String>,
) -> Result<Task, String> {
    let mut state = state.lock().unwrap();
    
    let task = state.tasks.iter_mut()
        .find(|t| t.id == task_id)
        .ok_or("任务不存在")?;

    // 更新提供的字段
    if let Some(t) = title {
        if t.is_empty() {
            return Err("任务标题不能为空".to_string());
        }
        task.title = t;
    }
    if let Some(time) = estimated_time {
        task.estimated_time = time;
    }
    if let Some(diff) = estimated_difficulty {
        if !(1..=5).contains(&diff) {
            return Err("预估难度必须在1~5之间".to_string());
        }
        task.estimated_difficulty = diff;
    }
    if let Some(important) = is_important {
        task.is_important = important;
    }
    if let Some(c) = comment {
        task.comment = Some(c);
    }

    let updated_task = task.clone();
    save_state(&state)?;

    Ok(updated_task)
}

/// 删除任务
#[command]
pub fn delete_task(
    state: tauri::State<'_, Mutex<AppState>>,
    task_id: u64,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    
    // 找到任务索引
    let task_index = state.tasks.iter()
        .position(|t| t.id == task_id)
        .ok_or("任务不存在")?;
    
    let task = state.tasks[task_index].clone();
    
    // 如果是子任务，从父任务的 sub_task_ids 中移除
    if let Some(parent_id) = task.parent_id {
        if let Some(parent) = state.tasks.iter_mut().find(|t| t.id == parent_id) {
            parent.sub_task_ids.retain(|&id| id != task_id);
        }
    }
    
    // 如果任务有子任务，先删除所有子任务
    for sub_id in &task.sub_task_ids {
        state.tasks.retain(|t| t.id != *sub_id);
    }
    
    // 删除任务
    state.tasks.remove(task_index);
    
    // 重新计算排序索引
    for (i, t) in state.tasks.iter_mut().enumerate() {
        t.order_index = i as u32;
    }
    
    save_state(&state)?;
    Ok(())
}

/// 完成任务
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
    if !(1..=5).contains(&energy_score) {
        return Err("精力分必须在1~5之间".to_string());
    }
    if !(1..=5).contains(&value_score) {
        return Err("价值分必须在1~5之间".to_string());
    }
    if !["😊", "😐", "😫", "🥳"].contains(&emoji_mood.as_str()) {
        return Err("心情表情必须是 😊/😐/😫/🥳 之一".to_string());
    }
    if let Some(ref c) = comment {
        if c.chars().count() > 20 {
            return Err("批注不能超过20个字".to_string());
        }
    }

    let mut state = state.lock().unwrap();
    let task = state.tasks.iter_mut()
        .find(|t| t.id == task_id)
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

    save_state(&state)?;
    Ok(())
}

// ==================== 查询命令 ====================

/// 按日期获取任务（创建或完成的）
#[command]
pub fn get_tasks_by_date(
    state: tauri::State<'_, Mutex<AppState>>,
    date_str: String,
) -> Result<Vec<Task>, String> {
    let state = state.lock().unwrap();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD".to_string())?;

    let tasks = state.tasks.iter()
        .filter(|t| {
            // 包括：当天创建的、当天完成的、当天分配的
            t.created_at == date 
                || t.completion_date == Some(date)
                || t.assigned_date == Some(date)
        })
        .cloned()
        .collect();

    Ok(tasks)
}

/// 获取所有未分配日期的任务（待办池）
#[command]
pub fn get_unassigned_tasks(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Task>, String> {
    let state = state.lock().unwrap();
    
    let mut tasks: Vec<Task> = state.tasks.iter()
        .filter(|t| t.assigned_date.is_none() && !t.is_completed && t.parent_id.is_none())
        .cloned()
        .collect();
    
    // 按 order_index 排序
    tasks.sort_by_key(|t| t.order_index);
    
    Ok(tasks)
}

/// 获取所有任务（包括子任务）
#[command]
pub fn get_all_tasks(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Task>, String> {
    let state = state.lock().unwrap();
    let mut tasks = state.tasks.clone();
    tasks.sort_by_key(|t| t.order_index);
    Ok(tasks)
}

/// 获取单个任务详情
#[command]
pub fn get_task_detail(
    state: tauri::State<'_, Mutex<AppState>>,
    task_id: u64,
) -> Result<Task, String> {
    let state = state.lock().unwrap();
    state.tasks.iter()
        .find(|t| t.id == task_id)
        .cloned()
        .ok_or_else(|| "任务不存在".to_string())
}

// ==================== 拖拽和日期分配 ====================

/// 将任务分配到指定日期（拖拽功能）
#[command]
pub fn assign_task_to_date(
    state: tauri::State<'_, Mutex<AppState>>,
    task_id: u64,
    date_str: Option<String>,
) -> Result<Task, String> {
    let mut state = state.lock().unwrap();
    
    let task = state.tasks.iter_mut()
        .find(|t| t.id == task_id)
        .ok_or("任务不存在")?;
    
    // 已完成的任务不能重新分配
    if task.is_completed {
        return Err("已完成的任务不能重新分配日期".to_string());
    }
    
    // 解析日期
    task.assigned_date = match date_str {
        Some(d) => Some(NaiveDate::parse_from_str(&d, "%Y-%m-%d")
            .map_err(|_| "日期格式错误")?),
        None => None,
    };
    
    let updated_task = task.clone();
    save_state(&state)?;
    
    Ok(updated_task)
}

/// 移动任务排序（拖拽排序）
#[command]
pub fn reorder_tasks(
    state: tauri::State<'_, Mutex<AppState>>,
    task_ids: Vec<u64>,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    
    // 验证所有任务存在
    for id in &task_ids {
        if !state.tasks.iter().any(|t| t.id == *id) {
            return Err(format!("任务 {} 不存在", id));
        }
    }
    
    // 更新排序索引
    for (index, task_id) in task_ids.iter().enumerate() {
        if let Some(task) = state.tasks.iter_mut().find(|t| t.id == *task_id) {
            task.order_index = index as u32;
        }
    }
    
    save_state(&state)?;
    Ok(())
}

// ==================== 子任务功能 ====================

/// 创建子任务
#[command]
pub fn create_sub_task(
    state: tauri::State<'_, Mutex<AppState>>,
    parent_id: u64,
    title: String,
    estimated_time: f32,
    estimated_difficulty: u8,
) -> Result<Task, String> {
    // 参数校验
    if title.is_empty() {
        return Err("任务标题不能为空".to_string());
    }
    if !(1..=5).contains(&estimated_difficulty) {
        return Err("预估难度必须在1~5之间".to_string());
    }

    let mut state = state.lock().unwrap();
    
    // 检查父任务是否存在
    if !state.tasks.iter().any(|t| t.id == parent_id) {
        return Err("父任务不存在".to_string());
    }
    
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
        is_important: false, // 子任务默认不是MIT
        emoji_mood: None,
        energy_score: None,
        value_score: None,
        comment: None,
        created_at: Local::now().date_naive(),
        parent_id: Some(parent_id),
        assigned_date: None,
        sub_task_ids: Vec::new(),
        order_index: 0, // 子任务不需要排序
    };

    // 添加到父任务的 sub_task_ids
    if let Some(parent) = state.tasks.iter_mut().find(|t| t.id == parent_id) {
        parent.sub_task_ids.push(id);
    }

    state.tasks.push(task.clone());
    save_state(&state)?;

    Ok(task)
}

/// 获取子任务列表
#[command]
pub fn get_sub_tasks(
    state: tauri::State<'_, Mutex<AppState>>,
    parent_id: u64,
) -> Result<Vec<Task>, String> {
    let state = state.lock().unwrap();
    
    // 检查父任务是否存在
    let parent = state.tasks.iter()
        .find(|t| t.id == parent_id)
        .ok_or("父任务不存在")?;
    
    let sub_tasks: Vec<Task> = parent.sub_task_ids.iter()
        .filter_map(|&id| state.tasks.iter().find(|t| t.id == id).cloned())
        .collect();
    
    Ok(sub_tasks)
}

/// 将子任务提升为独立任务
#[command]
pub fn promote_sub_task(
    state: tauri::State<'_, Mutex<AppState>>,
    task_id: u64,
) -> Result<Task, String> {
    let mut state = state.lock().unwrap();
    
    // 先找到任务索引和父任务ID
    let task_index = state.tasks.iter()
        .position(|t| t.id == task_id)
        .ok_or("任务不存在")?;
    
    let parent_id = state.tasks[task_index].parent_id
        .ok_or("该任务不是子任务")?;
    
    // 从父任务中移除
    if let Some(parent) = state.tasks.iter_mut().find(|t| t.id == parent_id) {
        parent.sub_task_ids.retain(|&id| id != task_id);
    }
    
    // 计算新的排序索引
    let new_order_index = state.tasks.len() as u32;
    
    // 提升为独立任务
    let task = state.tasks.get_mut(task_index).unwrap();
    task.parent_id = None;
    task.order_index = new_order_index;
    
    let updated_task = task.clone();
    save_state(&state)?;
    
    Ok(updated_task)
}