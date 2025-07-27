/*
实现可持续化存储
*/
use std::{error::Error, fs::File, io::Read, path::Path};
use crate::model::TodoItem;

const DATA_PATH: &str = "saved.json";

pub fn load_data() -> Result<Vec<TodoItem>, Box<dyn Error>> {
    if !Path::new(DATA_PATH).exists() {
        File::create(DATA_PATH)?;
    }
    let mut fileData = String::new();
    File::open(DATA_PATH)?.read_to_string(&mut fileData);

    // 空文件
    if fileData.trim().is_empty() {
        Ok(Vec::new())
    } else {
        Ok(serde_json::from_str(&fileData)?)
    }
}

pub fn save_data(item_list: &Vec<TodoItem>) -> Result<(), Box<dyn Error>> {
    Ok(())
}