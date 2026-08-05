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
}
fn parse_line_and_add_to_task_list(line: &str, tasks: &mut Vec<Task>) -> Option<()> {
    let mut add_task = |title: &str, state: TaskState| {
        tasks.push(Task {
            title: title.to_string(),
            state,
            subtasks: vec![],
        })
    };

    let valid_states = vec![
        (TaskState::Untouched),
        (TaskState::Done),
        (TaskState::Started),
        (TaskState::Skipped),
    ];

    let matched = valid_states.iter().find_map(|vs| {
        let rest_of_line = line.strip_prefix(vs.decoration())?;
        Some((rest_of_line, *vs))
    });

    match matched {
        Some(m) => {
            add_task(m.0, m.1);
            return Some(());
        }
        None => {
            let subtask_line = line.strip_prefix("    ");
            match subtask_line {
                Some(l) => {
                    let last_task = tasks.last_mut()?;
                    parse_line_and_add_to_task_list(l, &mut last_task.subtasks)?;
                    return Some(());
                }
                None => add_task(line, TaskState::Corrupted),
            }
        }
    }
    Some(())
}
