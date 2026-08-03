#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub state: TaskState,
    pub subtasks: Vec<Task>,
}

#[derive(Debug)]
pub enum TaskState {
    Untouched,
    Started,
    Postponed,
    Done,
}

impl Task {
    pub fn render(&self) -> String {
        self.render_level(0)
    }

    fn render_level(&self, level: u8) -> String {
        format!(
            "{:<5}{} [{}] {}{}",
            self.id,
            {
                let mut padding: String = String::new();
                for _ in 0..level + 1 {
                    padding += "    ";
                }
                padding
            },
            match self.state {
                TaskState::Untouched => " ",
                TaskState::Started => ".",
                TaskState::Postponed => "-",
                TaskState::Done => "x",
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
