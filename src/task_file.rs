use super::task::Task;
use super::task_state::TaskState;
use std::fs;

pub struct TaskFile {
    pub path: Option<String>,
    pub tasks: Vec<Task>,
}

impl TaskFile {

    pub fn from_file(file_path: &str) -> Option<TaskFile> {
        let content = fs::read_to_string(file_path).ok()?;
        let mut file = TaskFile::from_string(&content)?;
        file.path = Some(file_path.to_string());

        Some(file)
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
    pub fn save_as(mut self) -> Option<()>{
        todo!()
    }

    /// Saves file into the path defined on the TaskFile or returns None
    pub fn save_file(&self) -> Option<()> {
        todo!()
    }
}
fn parse_line_and_add_to_task_list(line: &str, tasks: &mut Vec<Task>) -> Option<()> {
    fn add_task(tasks: &mut Vec<Task>, title: &str, state: TaskState) {
        tasks.push(Task {
            title: title.to_string(),
            state,
            subtasks: vec![],
        })
    }

    let valid_states = [
        TaskState::Untouched,
        TaskState::Done,
        TaskState::Started,
        TaskState::Skipped,
    ];

    let matched = valid_states.iter().find_map(|vs| {
        let rest_of_line = line.strip_prefix(vs.decoration())?.strip_prefix(" ")?;
        Some((rest_of_line, *vs))
    });

    match matched {
        Some(m) => {
            add_task(tasks, m.0, m.1);
            return Some(());
        }
        None => {
            let subtask_line = line.strip_prefix("    ");
            match subtask_line {
                Some(l) => {
                    match tasks.last_mut() {
                        Some(last_task) => parse_line_and_add_to_task_list(l, &mut last_task.subtasks)?,
                        None => add_task(tasks, line, TaskState::Corrupted)
                    };
                    return Some(());
                }
                None => add_task(tasks, line, TaskState::Corrupted),
            }
        }
    }
    Some(())
}
