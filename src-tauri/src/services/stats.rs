use crate::models::{Task, DailyRating, AppState};
use crate::services::{
    generate_color,
    calculate_achievement_score,
};
use chrono::NaiveDate;

/// 获取出现次数最多的表情
fn get_most_frequent_emoji(moods: &[String]) -> String {
    if moods.is_empty() {
        return "😐".to_string();
    }
    
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for mood in moods {
        *counts.entry(mood.clone()).or_insert(0) += 1;
    }
    
    counts.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(mood, _)| mood)
        .unwrap_or_else(|| "😐".to_string())
}

/// 根据任务完成情况自动推断工作状态类型
fn infer_work_type(mit_rate: f32, avg_satisfaction: f32) -> String {
    if mit_rate >= 0.8 && avg_satisfaction >= 4.0 {
        "green".to_string()  // 高产日
    } else if mit_rate >= 0.5 {
        "blue".to_string()   // 学习日
    } else if mit_rate >= 0.3 {
        "orange".to_string() // 低效日
    } else {
        "gray".to_string()   // 休息/摸鱼日
    }
}

/// 更新某天的综合评分（根据当天完成的任务自动计算）
/// 当用户完成任务时自动调用，生成或更新该日的 DailyRating
pub fn update_daily_rating(state: &mut AppState, date: NaiveDate) {
    // 1. 收集当天所有完成的任务
    let completed_tasks: Vec<&Task> = state.tasks.iter()
        .filter(|t| t.completion_date == Some(date))
        .collect();

    // 2. 计算工作时长（实际耗时总和）
    let total_hours: f32 = completed_tasks.iter()
        .filter_map(|t| t.actual_time)
        .sum();

    // 3. 计算 MIT 完成率
    let completed_mit_count = completed_tasks.iter()
        .filter(|t| t.is_important)
        .count();
    let created_mit_count = state.tasks.iter()
        .filter(|t| t.created_at == date && t.is_important)
        .count();
    let mit_rate = if created_mit_count > 0 {
        completed_mit_count as f32 / created_mit_count as f32
    } else {
        0.0
    };

    // 4. 收集所有表情的能量和价值分
    let energy_scores: Vec<u8> = completed_tasks.iter()
        .filter_map(|t| t.energy_score)
        .collect();
    let value_scores: Vec<u8> = completed_tasks.iter()
        .filter_map(|t| t.value_score)
        .collect();
    
    // 计算平均 energy 和 value
    let avg_energy = if !energy_scores.is_empty() {
        energy_scores.iter().sum::<u8>() as f32 / energy_scores.len() as f32
    } else {
        3.0  // 默认值
    };
    let avg_value = if !value_scores.is_empty() {
        value_scores.iter().sum::<u8>() as f32 / value_scores.len() as f32
    } else {
        3.0  // 默认值
    };
    let avg_energy_u8 = avg_energy.round() as u8;
    let avg_value_u8 = avg_value.round() as u8;

    // 5. 情绪表情（取出现次数最多的）
    let moods: Vec<String> = completed_tasks.iter()
        .filter_map(|t| t.emoji_mood.clone())
        .collect();
    let mood_emoji = get_most_frequent_emoji(&moods);

    // 6. 计算平均满意度
    let satisfactions: Vec<u8> = completed_tasks.iter()
        .filter_map(|t| t.actual_satisfaction)
        .collect();
    let avg_satisfaction = if !satisfactions.is_empty() {
        satisfactions.iter().sum::<u8>() as f32 / satisfactions.len() as f32
    } else {
        3.0
    };

    // 7. 自动推断工作状态类型
    let work_type = infer_work_type(mit_rate, avg_satisfaction);

    // 8. 计算成就分（使用共享计算模块）
    let achievement_score = calculate_achievement_score(avg_energy_u8, avg_value_u8);

    // 9. 生成颜色（使用共享计算模块）
    let productivity_color = generate_color(&work_type, avg_energy_u8, avg_value_u8);

    // 10. 计算任务量变化（与昨天对比）
    let yesterday = date - chrono::Duration::days(1);
    let today_count = state.tasks.iter()
        .filter(|t| t.created_at == date)
        .count();
    let yesterday_count = state.tasks.iter()
        .filter(|t| t.created_at == yesterday)
        .count();
    let task_count_delta = today_count as i8 - yesterday_count as i8;

    // 11. 检查是否有逾期（简化实现：有未完成的任务且创建日期早于今天）
    let has_overdue = state.tasks.iter()
        .any(|t| !t.is_completed && t.created_at < date);

    // 12. 里程碑判断（MIT完成率100%且至少完成3个任务）
    let has_milestone = mit_rate >= 1.0 && completed_tasks.len() >= 3;

    // 13. 创建或更新评分记录
    let rating = DailyRating {
        date,
        work_type,
        energy_score: avg_energy_u8,
        value_score: avg_value_u8,
        work_hours: total_hours,
        mood_emoji,
        comment: None, // 自动生成的记录没有评语
        productivity_color,
        achievement_score,
        status: 0,
        mit_completion_rate: mit_rate,
        task_count_delta,
        has_overdue,
        has_milestone,
    };

    state.daily_ratings.insert(date, rating);
}

/// 合并自动计算和手动输入的数据
/// 当用户手动保存评分时，保留自动计算的统计字段，更新用户输入的字段
pub fn merge_daily_rating(auto_rating: &DailyRating, manual_rating: DailyRating) -> DailyRating {
    // 先计算颜色和成就分（使用共享计算模块）
    let productivity_color = generate_color(
        &manual_rating.work_type,
        manual_rating.energy_score,
        manual_rating.value_score
    );
    let achievement_score = calculate_achievement_score(
        manual_rating.energy_score,
        manual_rating.value_score
    );
    
    DailyRating {
        date: manual_rating.date,
        // 用户输入的字段
        work_type: manual_rating.work_type,
        energy_score: manual_rating.energy_score,
        value_score: manual_rating.value_score,
        work_hours: manual_rating.work_hours,
        mood_emoji: manual_rating.mood_emoji,
        comment: manual_rating.comment,
        // 计算生成的字段
        productivity_color,
        achievement_score,
        // 保留自动计算的统计字段
        status: auto_rating.status,
        mit_completion_rate: auto_rating.mit_completion_rate,
        task_count_delta: auto_rating.task_count_delta,
        has_overdue: auto_rating.has_overdue,
        has_milestone: auto_rating.has_milestone,
    }
}