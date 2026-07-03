mod err;
use std::fs;

use std::error::Error;
use err::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(err) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(err),
                }))
            }
        };

        let tasks = match json::parse(&content) {
            Ok(data) => data,
            Err(err) => return Err(Box::new(ParseErr::Malformed(Box::new(err)))),
        };

        if tasks["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty));
        };

        let mut tasks_list = Vec::new();

        for task in tasks["tasks"].members() {
            tasks_list.push(Task {
                id: task["id"].as_u32().unwrap(),
                description: task["description"].as_str().unwrap().to_string(),
                level: task["level"].as_u32().unwrap(),
            });
        };

        Ok(TodoList {
            title: tasks["title"].as_str().unwrap_or("").to_string(),
            tasks: tasks_list,
        })

    }
}