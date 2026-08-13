use crossterm::style::Stylize;

use crate::color::to_crossterm_color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Untouched,
    Started,
    Skipped,
    Done,
    NonTask,
}

impl TaskState {
    pub fn decoration(&self) -> &str {
        match self {
            TaskState::Untouched => "[ ]",
            TaskState::Started => "[.]",
            TaskState::Done => "[x]",
            TaskState::Skipped => "[-]",
            TaskState::NonTask => " \\_ ",
        }
    }

    pub fn decoration_color(&self, color: (u8, u8, u8)) -> String {
        match self {
            TaskState::Untouched => "[ ]".grey().to_string(),
            TaskState::Started => {
                "[".grey().to_string()
                    + &".".with(to_crossterm_color(color)).to_string()
                    + &"]".grey().to_string()
            }
            TaskState::Done => {
                "[".grey().to_string()
                    + &"x".with(to_crossterm_color(color)).to_string()
                    + &"]".grey().to_string()
            }
            TaskState::Skipped => {
                "[".grey().to_string()
                    + &"-".with(to_crossterm_color(color)).to_string()
                    + &"]".grey().to_string()
            }
            TaskState::NonTask => " \\_".dark_grey().to_string(),
        }
    }
}
