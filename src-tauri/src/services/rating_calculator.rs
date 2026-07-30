//! 评分计算模块
//! 集中管理所有评分相关的计算逻辑，避免代码重复

/// 根据工作状态类型获取基础色相（RGB）
pub fn work_type_to_rgb(work_type: &str) -> (u8, u8, u8) {
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
pub fn calculate_saturation(energy_score: u8, value_score: u8) -> f32 {
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
pub fn generate_color(work_type: &str, energy_score: u8, value_score: u8) -> String {
    let (r, g, b) = work_type_to_rgb(work_type);
    let alpha = calculate_saturation(energy_score, value_score);
    format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha)
}

/// 计算成就分数（0-100）
/// 基于效率指数和价值分
pub fn calculate_achievement_score(energy_score: u8, value_score: u8) -> u8 {
    if energy_score == 0 || value_score == 0 {
        return 0;
    }
    // 效率指数 * 价值权重
    let efficiency = value_score as f32 / energy_score as f32;
    let score = (efficiency * value_score as f32 * 4.0).min(100.0) as u8;
    score
}

/// 计算效率指数（用于其他统计）
pub fn calculate_efficiency(energy_score: u8, value_score: u8) -> f32 {
    if energy_score == 0 {
        return 0.0;
    }
    value_score as f32 / energy_score as f32
}

/// 根据效率指数获取评价标签
pub fn get_efficiency_label(efficiency: f32) -> &'static str {
    match efficiency {
        e if e >= 2.0 => "超高效率",
        e if e >= 1.5 => "高效率",
        e if e >= 1.0 => "正常效率",
        e if e >= 0.5 => "低效率",
        _ => "极低效率",
    }
}