/*
实现cli的方法
维护数据
 */
use std::error::Error;
use crate::model::TodoItem;
use crate::storage::{ load_data, save_data };

struct TaskManager {
    tasks: Vec<TodoItem>,
}

impl TaskManager {
    // 创建一个新的 Task Manager 实例
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let tasks: Vec<TodoItem> = load_data()?;
        Ok(TaskManager { tasks: tasks })
    }

    pub fn add_item(&mut self, new_data: TodoItem) {
        self.tasks.push(new_data);
    }

    pub fn delete_item(&mut self, id: u32) -> Result<bool, Box<dyn Error>>{
        let origin_len = self.tasks.len();
        self.tasks.retain(|x| x.id != id);

        if (self.tasks.len() >= origin_len) {
            Ok(false)
        } else {
            save_data(&self.tasks)?;
            Ok(true)
        }
    }
}

// 定义 TodoList 中所有的操作方式
pub enum TodoListCommand {
    Add { todo_item: TodoItem },
    Delete,
    Done,
    ShowSingle { index: u32 },
    ShowAll,
}