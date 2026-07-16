use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub tasks: Vec<Task>,
    pub daily_ratings: HashMap<NaiveDate, DailyRating>,
    pub next_task_id: u64,
}