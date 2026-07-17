use crate::models::{Task, DailyRating, AppState};
use chrono::NaiveDate;

/// 更新某天的综合评分
pub fn update_daily_rating(state: &mut AppState, date: NaiveDate) {
    // 1. 收集当天所有完成的任务
    let tasks: Vec<&Task> = state.tasks.iter()
        .filter(|t| t.completion_date == Some(date))
        .collect();

    if tasks.is_empty() {
        // 没有任务，则设置默认灰色
        let rating = DailyRating {
            date,
            productivity_color: "gray".to_string(),
            work_hours: 0.0,
            mood_emoji: "😐".to_string(),
            achievement_score: 0,
            mit_completion_rate: 0.0,
            task_count_delta: 0,
            has_overdue: false,
            has_milestone: false,
            comments: vec![],
        };
        state.daily_ratings.insert(date, rating);
        return;
    }

    // 2. 计算工作时长（实际耗时总和）
    let total_hours: f32 = tasks.iter().filter_map(|t| t.actual_time).sum();

    // 3. 计算 MIT 完成率
    let mit_tasks: Vec<&Task> = tasks.iter().filter(|t| t.is_important).cloned().collect();
    let mit_complete = mit_tasks.len();
    // 这里需要知道当天有多少 MIT 任务被创建，但我们没有单独记录，可以用一个计数器，或者根据任务创建日期推算。
    // 简化为：当天完成的所有重要任务 / 当天创建的所有重要任务。需另写函数计算当天创建的重要任务数。
    // 我们暂时使用一个简单方式：获取当天创建的所有重要任务，计算完成率。
    let created_mit: usize = state.tasks.iter()
        .filter(|t| t.created_at == date && t.is_important)
        .count();
    let mit_rate = if created_mit > 0 { mit_complete as f32 / created_mit as f32 } else { 0.0 };

    // 4. 确定颜色（绿/蓝/橙/灰）—— 可根据完成率和满意度综合判断，这里简化
    let color = if mit_rate >= 0.8 { "green" } else if mit_rate >= 0.5 { "blue" } else { "orange" };

    // 5. 情绪表情（取出现次数最多的）
    let moods: Vec<String> = tasks.iter().filter_map(|t| t.emoji_mood.clone()).collect();
    let mood_emoji = if moods.is_empty() {
        "😐".to_string()
    } else {
        // 统计频次（简化，取第一个）
        moods[0].clone()
    };

    // 6. 成就分（综合评分，比如 0~100）
    let score = (mit_rate * 100.0) as u8;

    // 7. 计算任务量变化（与昨天对比）
    let yesterday = date - chrono::Duration::days(1);
    let today_count = state.tasks.iter().filter(|t| t.created_at == date).count();
    let yesterday_count = state.tasks.iter().filter(|t| t.created_at == yesterday).count();
    let delta = today_count as i8 - yesterday_count as i8;

    // 8. 检查是否有逾期（这里实现简单：若有未完成任务，且截止日期已过，但暂时没有截止日期字段，可先忽略）
    let has_overdue = false; // 可扩展

    // 9. 里程碑（暂时没有，可后续添加）

    let rating = DailyRating {
        date,
        productivity_color: color.to_string(),
        work_hours: total_hours,
        mood_emoji,
        achievement_score: score,
        mit_completion_rate: mit_rate,
        task_count_delta: delta,
        has_overdue,
        has_milestone: false,
        comments: vec![],
    };

    state.daily_ratings.insert(date, rating);
}