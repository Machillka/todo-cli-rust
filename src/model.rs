use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Local};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: u32,
    pub content: String,
    pub is_done: bool,
    pub time_create_at: DateTime<Local>,
}