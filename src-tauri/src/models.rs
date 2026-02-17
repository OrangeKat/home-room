use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub date: String,
    pub time: String,
    #[serde(rename = "isUrgent", alias = "is_urgent")]
    pub is_urgent: bool,
    #[serde(alias = "is_completed")]
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub date: String,
    pub time: String,
    #[serde(rename = "is_urgent")]
    pub is_urgent: bool,
    #[serde(rename = "is_completed")]
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Habit {
    pub id: i64,
    pub title: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewHabit {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabitCompletion {
    pub habit_id: i64,
    pub completed_date: String,
}
