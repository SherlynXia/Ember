// 提供前端所需的热力图数据
#[command]
pub fn get_heatmap_data(
    state: tauri::State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<Vec<f64>>, String> {
    // 根据日期范围，返回一个二维数组（行=日期，列=任务或时段）
    // 数据可以是预估-实际偏差、评分等，前端决定如何展示
    todo!()
}