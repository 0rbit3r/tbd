use super::task_state::TaskState;

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub state: TaskState,
    pub subtasks: Vec<Task>,
}

impl Task {
    pub fn render(&self) -> String {
        self.render_level(0)
    }

    fn render_level(&self, level: u8) -> String {
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
                    result = result + "\n" + &subtask.render_level(level + 1);
                }
                result
            }
        )
    }
}
