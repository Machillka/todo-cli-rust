use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: u32,
    pub content: String,
    pub is_done: bool,
}