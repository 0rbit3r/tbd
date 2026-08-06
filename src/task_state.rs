#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Untouched,
    Started,
    Skipped,
    Done,
    Corrupted,
}

impl TaskState {
    pub fn decoration(&self) -> &str {
        match self {
            TaskState::Untouched => "[ ]",
            TaskState::Started => "[.]",
            TaskState::Done => "[x]",
            TaskState::Skipped => "[-]",
            TaskState::Corrupted => "Err",
        }
    }
}
