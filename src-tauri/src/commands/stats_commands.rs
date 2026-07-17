// 提供前端所需的热力图数据
use tauri::command;
use chrono::Duration;
use crate::models::AppState;

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

    // 构造二维数组：行=日期，列=任务（按创建顺序），数据根据 data_type 不同
    // 这里需要遍历日期范围，再遍历当天的任务，提取相应数据
    // 比较复杂，可先返回一个示例结构
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