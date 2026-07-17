use super::{Task, DailyRating};
use std::collections::HashMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub tasks: Vec<Task>,
    pub daily_ratings: HashMap<NaiveDate, DailyRating>,
    pub next_task_id: u64,
}