mod get_task;
mod indentation;
mod indexing;
mod insert;
mod parse;
mod remove;

use crate::task_state::TaskState;
use get_task::get_task;
use get_task::get_task_mut;
use indentation::indent_task_r;
use indentation::unindent_task_r;
use remove::remove_task_r;

use super::task::Task;
pub use indexing::{IndexRes, MultiIndexRes, index_to_multi_index, multi_index_to_index};
use insert::*;
use parse::*;
use std::error::Error;
use std::fs;

pub struct TaskFile {
    pub path: Option<String>,
    pub tasks: Vec<Task>,
}

impl TaskFile {
    pub fn new() -> TaskFile {
        TaskFile {
            path: None,
            tasks: vec![],
        }
    }

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

    pub fn render_file(&self) -> String {
        let mut lines: Vec<String> = vec![];
        for task in &self.tasks {
            lines.push(task.render_file())
        }
        lines.join("\n")
    }

    /// Saves file into the path defined on the TaskFile or returns None
    pub fn save_file(&self) -> Result<(), Box<dyn Error>> {
        match &self.path {
            None => return Err("This task-file has no file associated. Use save_as.".into()),
            Some(path) => {
                let content = self.render_file();
                fs::write(path, content)?;
            }
        }
        Ok(())
    }

    // Will insert a new task under the specified index
    // - either as a sibling when the task at index has no subtasks or as subtask otherwise
    // None index will push new top-level task to the end
    pub fn insert_task(&mut self, new_task: Task, index: Option<usize>) -> Option<()> {
        match index {
            None => {
                self.tasks.push(new_task);
                Some(())
            }
            Some(i) => match index_to_multi_index(&self.tasks, i) {
                MultiIndexRes::NotFound(_) => None,
                MultiIndexRes::Found(multi_index) => {
                    insert_task_to_task_tree(&mut self.tasks, new_task, &multi_index)
                }
            },
        }
    }

    pub fn get_task_at(&self, index: usize) -> Option<&Task> {
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };
        get_task(&self.tasks, &index)
    }

    pub fn get_task_at_mut(&mut self, index: usize) -> Option<&mut Task> {
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };
        get_task_mut(&mut self.tasks, &index)
    }

    /// # Examples
    /// ```
    /// let mut task_file = tbd::TaskFile::from_string("[ ] Untouched").unwrap();
    /// task_file.mark_as(0, tbd::TaskState::Done);
    /// assert_eq!(tbd::TaskState::Done, task_file.tasks[0].state);
    /// ```
    /// ```
    /// let mut task_file = tbd::TaskFile::from_string("[ ] Untouched").unwrap();
    /// assert_eq!(None, task_file.mark_as(1, tbd::TaskState::Done));
    /// ```
    pub fn mark_as(&mut self, index: usize, new_state: TaskState) -> Option<()> {
        let task = self.get_task_at_mut(index)?;
        task.state = new_state;
        Some(())
    }

    pub fn indent_task(&mut self, index: usize) -> Option<()> {
        if index == 0 {
            return None;
        };
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };
        indent_task_r(&mut self.tasks, &index)
    }

    ///returns how many lines down the task traveled
    pub fn unindent_task(&mut self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        };
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };
        unindent_task_r(&mut self.tasks, &index)
    }

    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };
        remove_task_r(&mut self.tasks, &index)
    }

    pub fn move_task_up(&mut self, index: usize) -> Option<()> {
        if index < 1 {
            return None;
        }
        if let Some(task) = self.remove_task(index) {
            if index == 1 {
                self.tasks.insert(0, task);
            } else {
                self.insert_task(task, Some(index - 2));
            }
        };
        Some(())
    }
    pub fn move_task_down(&mut self, index: usize) -> Option<()> {
        // if index > tasks_count {
        //     return None;
        // } todo - get_size
        if let Some(task) = self.remove_task(index) {
            self.insert_task(task, Some(index));
        };
        Some(())
    }
}

impl Default for TaskFile {
    fn default() -> TaskFile {
        Self::new()
    }
}
