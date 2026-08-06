mod indexing;
mod insert;
mod parse;

use self::indexing::*;
use self::insert::*;
use self::parse::*;
use super::task::Task;
use std::error::Error;
use std::fs;

pub struct TaskFile {
    pub path: Option<String>,
    pub tasks: Vec<Task>,
}

impl TaskFile {
    pub fn from_file(file_path: &str) -> Result<TaskFile, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut file = TaskFile::from_string(&content).ok_or("Failed to parse file.")?;
        file.path = Some(file_path.to_string());

        Ok(file)
    }

    pub fn from_string(content: &str) -> Option<TaskFile> {
        //Use result instead

        let lines = content.lines();

        let mut result = vec![];

        for line in lines {
            parse_line_and_add_to_task_list(line, &mut result);
        }

        Some(TaskFile {
            path: None,
            tasks: result,
        })
    }

    /// Saves the task file as a new file - will overwrite the path property
    pub fn save_as(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        self.path = Some(path.to_string());

        self.save_file()
    }

    pub fn render_screen(&self) -> String {
        let mut lines: Vec<String> = vec![];
        for task in &self.tasks {
            lines.push(task.render(true))
        }
        lines.join("\n")
    }

    pub fn render_file(&self) -> String {
        let mut lines: Vec<String> = vec![];
        for task in &self.tasks {
            lines.push(task.render(false))
        }
        lines.join("\n")
    }

    /// Saves file into the path defined on the TaskFile or returns None
    pub fn save_file(&self) -> Result<(), Box<dyn Error>> {
        match &self.path {
            None => return Err("This task-file has no file associated. Use save_as.".into()),
            Some(path) => {
                let content = self.render_screen();
                fs::write(path, content)?;
            }
        }
        Ok(())
    }

    pub fn insert_task(&mut self, new_task: Task, index: Option<usize>) {
        match index {
            None => {
                self.tasks.push(new_task);
            }
            Some(i) => match self.get_multi_index(i) {
                MultiIndexRes::NotFound(_) => return,
                MultiIndexRes::Found(multi_index) => {
                    let insert_result =
                        insert_task_to_task_tree(&mut self.tasks, new_task, &multi_index);
                    println!("{insert_result:?}");
                }
            },
        }
    }

    pub fn get_multi_index(&self, index: usize) -> MultiIndexRes {
        index_to_multi_index(&self.tasks, index)
    }
}
