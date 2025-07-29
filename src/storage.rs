/*
实现可持续化存储
*/
use std::{error::Error, fs::{File, OpenOptions}, io::{Read, Write}, path::Path};
use crate::model::TodoItem;

const DATA_PATH: &str = "saved.json";

pub fn load_data() -> Result<Vec<TodoItem>, Box<dyn Error>> {
    if !Path::new(DATA_PATH).exists() {
        File::create(DATA_PATH)?;
    }
    let mut file_data = String::new();
    File::open(DATA_PATH)?.read_to_string(&mut file_data)?;

    // 空文件
    if file_data.trim().is_empty() {
        Ok(Vec::new())
    } else {
        Ok(serde_json::from_str(&file_data)?)
    }
}

pub fn save_data(item_list: &Vec<TodoItem>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(item_list)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(DATA_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}