mod calc;
mod stats;
pub mod rating_calculator;

pub use calc::calculate_deviation;
pub use stats::{update_daily_rating, merge_daily_rating};
pub use rating_calculator::{
    work_type_to_rgb,
    calculate_saturation,
    generate_color,
    calculate_achievement_score,
    calculate_efficiency,
    get_efficiency_label,
};