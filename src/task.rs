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
    pub fn render_file(&self) -> String {
        self.render_file_level(0)
    }

    pub fn get_count(&self) -> usize {
        return if self.subtasks.is_empty() {
            1
        } else {
            1 + self.subtasks.iter().fold(0, |f, t| f + t.get_count())
        };
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
                TaskState::Corrupted => "".to_string(),
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

    pub fn render_screen(&self) -> String {
        self.render_screen_level(0)
    }

    fn render_screen_level(&self, level: u8) -> String {
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
                TaskState::Corrupted => "Err ".to_string(),
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
                    result = result + "\n" + &subtask.render_screen_level(level + 1);
                }
                result
            }
        )
    }
}
