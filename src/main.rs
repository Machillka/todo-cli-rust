mod model;
mod manager;
mod storage;
use std::{error::Error};

use manager::TaskManager;
use clap::{ Subcommand, Parser };

#[derive(Parser)]
#[command(name = "task_cli")]
struct Cli {
    #[command(subcommand)]
    action: TodoListCommand,
}

// 定义 TodoList 中所有的操作方式
#[derive(Subcommand)]
pub enum TodoListCommand {
    Add { title: String },
    Delete { id: u32 },
    Done { id: u32},
    // ShowSingle { id: u32},
    ShowAll,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut manager = TaskManager::new()?;

    match cli.action {
        TodoListCommand::Add { title } => {
            let id = manager.add_item(title)?;
            println!("成功添加任务 #{}", id);
        },
        TodoListCommand::ShowAll => {
            manager.show_all_tasks();
        },
        TodoListCommand::Delete { id } => {
            manager.delete_item(id)?;
            println!("成功删除任务 #{}", id);
        },
        TodoListCommand::Done { id } => {
            if manager.sign_done(id)? {
                println!("标记成功");
            } else {
                println!("标记失败");
            }
        }
    }

    Ok(())
}
/*
 实现 cli 下的 todo list
 实现功能: 添加、删除、修改、查看
 读取参数列表，进行功能的处理
 */