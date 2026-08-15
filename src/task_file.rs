mod get_task;
mod indentation;
mod indexing;
mod insert;
mod move_task;
mod parse;
mod remove;

use crate::task_state::TaskState;
use get_task::get_task_mut_r;
use get_task::get_task_r;
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
    pub path: String,
    pub tasks: Vec<Task>,
    pub saved: bool,
}

impl TaskFile {
    pub fn new(path: &str) -> TaskFile {
        TaskFile {
            path: path.to_string(),
            tasks: vec![],
            saved: false,
        }
    }

    pub fn from_file(file_path: &str) -> Result<TaskFile, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut file = TaskFile::from_string(file_path, &content).ok_or("Failed to parse file.")?;
        file.saved = true;
        Ok(file)
    }

    pub fn from_string(path: &str, content: &str) -> Option<TaskFile> {
        //Use result instead

        let lines = content.lines();

        let mut result = vec![];

        for line in lines {
            parse_line_and_add_to_task_list(line, &mut result);
        }

        Some(TaskFile {
            path: path.to_string(),
            tasks: result,
            saved: false,
        })
    }

    pub fn render_file(&self) -> String {
        let mut lines: Vec<String> = vec![];
        for task in &self.tasks {
            lines.push(task.render_file())
        }
        lines.join("\n")
    }

    /// Saves file into the path defined on the TaskFile or returns None
    pub fn save_file(&mut self) -> Result<(), Box<dyn Error>> {
        let content = self.render_file();
        fs::write(&self.path, content)?;
        self.saved = true;
        Ok(())
    }

    // Will insert a new task under the specified index
    // - either as a sibling when the task at index has no subtasks or as subtask otherwise
    // None index will push new top-level task to the end
    pub fn insert_task(&mut self, new_task: Task, index: Option<usize>) -> Option<()> {
        if self.tasks_count() == 0 {
            self.tasks.push(new_task);
            self.saved = false;
            return Some(());
        }
        match index {
            None => {
                self.tasks.push(new_task);
                self.saved = false;
                Some(())
            }
            Some(i) => match index_to_multi_index(&self.tasks, i) {
                MultiIndexRes::NotFound(_) => None,
                MultiIndexRes::Found(multi_index) => {
                    insert_task_to_task_tree_r(&mut self.tasks, new_task, &multi_index)
                }
            },
        }
    }

    pub fn get_task_at(&self, index: usize) -> Option<&Task> {
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };
        get_task_r(&self.tasks, &index)
    }

    pub fn get_task_at_mut(&mut self, index: usize) -> Option<&mut Task> {
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };

        if let Some(t) = get_task_mut_r(&mut self.tasks, &index) {
            self.saved = false;
            Some(t)
        } else {
            None
        }
    }

    /// # Examples
    /// ```
    /// let mut task_file = tbd::TaskFile::from_string("tasks","[ ] Untouched").unwrap();
    /// task_file.mark_as(0, tbd::TaskState::Done);
    /// assert_eq!(tbd::TaskState::Done, task_file.tasks[0].state);
    /// ```
    /// ```
    /// let mut task_file = tbd::TaskFile::from_string("tasks", "[ ] Untouched").unwrap();
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
        if let Some(()) = indent_task_r(&mut self.tasks, &index) {
            self.saved = false;
            Some(())
        } else {
            None
        }
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

        if let Some(s) = unindent_task_r(&mut self.tasks, &index) {
            self.saved = false;
            Some(s)
        } else {
            None
        }
    }

    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        let index = match index_to_multi_index(&self.tasks, index) {
            MultiIndexRes::Found(mi) => mi,
            MultiIndexRes::NotFound(_) => return None,
        };

        if let Some(t) = remove_task_r(&mut self.tasks, &index) {
            self.saved = false;
            Some(t)
        } else {
            None
        }
    }

    pub fn tasks_count(&self) -> usize {
        self.tasks.iter().map(|t| t.get_count()).sum()
    }
}
