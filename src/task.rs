use super::task_state::TaskState;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub title: String,
    pub state: TaskState,
    pub subtasks: Vec<Task>,
}

impl Task {
    pub fn render(&self, mark_errors: bool) -> String {
        self.render_level(0, mark_errors)
    }

    fn render_level(&self, level: u8, mark_errors: bool) -> String {
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
                TaskState::Corrupted =>
                    if mark_errors {
                        "Err ".to_string()
                    } else {
                        "".to_string()
                    },
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
                    result = result + "\n" + &subtask.render_level(level + 1, mark_errors);
                }
                result
            }
        )
    }
}
