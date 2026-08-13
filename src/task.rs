use crossterm::style::Stylize;

use crate::color::{get_lighter, to_crossterm_color};

use super::task_state::TaskState;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub title: String,
    pub state: TaskState,
    pub subtasks: Vec<Task>,
}

impl Task {
    pub fn new() -> Task {
        Task {
            title: String::new(),
            state: TaskState::Untouched,
            subtasks: vec![],
        }
    }

    pub fn get_count(&self) -> usize {
        if self.subtasks.is_empty() {
            1
        } else {
            1 + self.subtasks.iter().fold(0, |f, t| f + t.get_count())
        }
    }

    pub fn render_file(&self) -> String {
        self.render_file_level(0)
    }

    fn render_file_level(&self, level: usize) -> String {
        format!(
            "{}{}{}{}",
            {
                let mut padding: String = String::new();
                for _ in 0..level {
                    padding += "    ";
                }
                padding
            },
            match self.state {
                TaskState::NonTask => "".to_string(),
                _ => {
                    let mut with_space = self.state.decoration().to_string();
                    with_space += " ";
                    with_space
                }
            },
            self.title,
            {
                let mut result: String = String::new();
                for subtask in &self.subtasks {
                    result = result + "\n" + &subtask.render_file_level(level + 1);
                }
                result
            }
        )
    }

    pub fn render_screen(&self, color: (u8, u8, u8)) -> String {
        self.render_screen_level(0, color)
    }

    fn render_screen_level(&self, level: u8, color: (u8, u8, u8)) -> String {
        format!(
            "{}{}{}{}",
            {
                let mut padding: String = String::new();
                for _ in 0..level {
                    padding += "    ";
                }
                padding
            },
            {
                let mut with_space = self.state.decoration_color(color).to_string();
                with_space += " ";
                with_space
            },{
            let mut color = color;
            for _ in 0..level {color = get_lighter(color);}
            self.title
                .clone()
                .with(to_crossterm_color(if self.state == TaskState::NonTask {(150,150,150)} else {color}))
                .to_string()
            },{
                let mut result: String = String::new();
                for subtask in &self.subtasks {
                    result = result + "\n" + &subtask.render_screen_level(level + 1, color);
                }
                result
            }
        )
    }
}

impl Default for Task {
    fn default() -> Task {
        Task::new()
    }
}
