/*
实现cli的方法
维护数据
 */
use std::error::Error;

use chrono::Local;

use crate::model::TodoItem;
use crate::storage::{ load_data, save_data };

pub struct TaskManager {
    tasks: Vec<TodoItem>,
}

impl TaskManager {
    // 创建一个新的 Task Manager 实例
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let tasks: Vec<TodoItem> = load_data()?;
        Ok(TaskManager { tasks: tasks })
    }

    pub fn add_item(&mut self, content: String) -> Result<u32, Box<dyn Error>> {
        let id = self.tasks.last().map_or(1, |task| task.id + 1);
        self.tasks.push(TodoItem {
            id: id,
            content: content,
            is_done: false,
            time_create_at: Local::now(),
        });
        save_data(&self.tasks)?;
        Ok(id)
    }

    pub fn delete_item(&mut self, id: u32) -> Result<bool, Box<dyn Error>>{
        let origin_len = self.tasks.len();
        self.tasks.retain(|x| x.id != id);

        if self.tasks.len() >= origin_len {
            Ok(false)
        } else {
            save_data(&self.tasks)?;
            Ok(true)
        }
    }

    pub fn sign_done(&mut self, id: u32) -> Result<bool, Box<dyn Error>> {
        // // TODO: 使用迭代器实现功能
        // for task in &mut self.tasks {
        //     // NOTE: 为什么这里不需要解引用？是因为复杂类型吗
        //     if (task.id == id) {
        //         task.is_done = !task.is_done;
        //     }
        // }
        if let Some(task) = self.tasks.iter_mut().find(|task: &&mut TodoItem| task.id == id) {
            task.is_done = !task.is_done;
            save_data(&self.tasks)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // pub fn show_single(&self, id: u32) -> Option<bool> {

    // }

    pub fn show_all_tasks(&self) {
        use colored::Colorize;
        use std::io::Write;
        use tabwriter::TabWriter;

        if self.tasks.is_empty() {
            println!("No Todos");
            return;
        }

        let mut tw = TabWriter::new(std::io::stdout())
            .padding(2)
            .minwidth(4);

        writeln!(tw, "ID\t标题\t创建时间\t状态").unwrap();
        for t in &self.tasks {
            let time_str = t.time_create_at.format("%Y-%m-%d %H:%M:%S");
            let status = if t.is_done {
                "✅ 已完成".green()
            } else {
                "⏳ 未完成".yellow()
            };
            writeln!(
                tw,
                "{}\t{}\t{}\t{}",
                t.id, t.content, time_str, status
            )
            .unwrap();
        }
        tw.flush().unwrap();
    }
}