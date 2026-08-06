use super::task::Task;
use super::task_state::TaskState;
use std::error::Error;
use std::fs;
use std::iter::Enumerate;

pub struct TaskFile {
    pub path: Option<String>,
    pub tasks: Vec<Task>,
}

impl TaskFile {
    pub fn from_file(file_path: &str) -> Result<TaskFile, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut file = TaskFile::from_string(&content).ok_or("Failed to parse file.")?;
        file.path = Some(file_path.to_string());

        Ok(file)
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
    pub fn save_as(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        self.path = Some(path.to_string());

        self.save_file()
    }

    /// Saves file into the path defined on the TaskFile or returns None
    pub fn save_file(&self) -> Result<(), Box<dyn Error>> {
        match &self.path {
            None => return Err("This task-file has no file associated. Use save_as.".into()),
            Some(path) => {
                let mut content = String::new();

                for task in &self.tasks {
                    content += &task.render();
                    content += "\n";
                }

                fs::write(path, content)?;
            }
        }

        Ok(())
    }

    pub fn insert_task(&mut self, new_task: Task, index: Option<usize>) {
        let mut root = &mut self.tasks;
        match index {
            None => {
                root.push(new_task);
            }
            Some(i) => match index_to_multi_index(root, i) {
                MultiIndexRes::NotFound(_) => return,
                MultiIndexRes::Found(multi_index) => {
                    let insert_result = insert_task_to_task_tree(root, new_task, &multi_index);
                    println!("{insert_result:?}");
                }
            },
        }
    }
}

/// This function will parse the string into provided Tasks vector.
/// In case of syntax errors, affected lines will be added as Malformed tasks
fn parse_line_and_add_to_task_list(line: &str, tasks: &mut Vec<Task>) {
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
        }
        None => {
            let subtask_line = line.strip_prefix("    ");
            match subtask_line {
                Some(l) => {
                    match tasks.last_mut() {
                        Some(last_task) => {
                            parse_line_and_add_to_task_list(l, &mut last_task.subtasks)
                        }
                        None => add_task(tasks, line, TaskState::Corrupted),
                    };
                }
                None => add_task(tasks, line, TaskState::Corrupted),
            }
        }
    };
}

#[derive(Debug)]
enum MultiIndexRes {
    NotFound(usize),
    Found(Vec<usize>),
}

fn index_to_multi_index(task_list: &Vec<Task>, desired_index: usize) -> MultiIndexRes {
    let desired_index_orig = desired_index;
    let mut desired_index = desired_index;
    if task_list.is_empty() {
        return MultiIndexRes::NotFound(0);
    };

    for (local_i, task) in task_list.iter().enumerate() {
        if desired_index == 0 {
            return MultiIndexRes::Found(vec![local_i]);
        }

        desired_index -= 1;
        match index_to_multi_index(&task.subtasks, desired_index) {
            MultiIndexRes::NotFound(size) => {
                desired_index -= size;
            }
            MultiIndexRes::Found(mut indexes) => {
                indexes.insert(0, local_i);
                return MultiIndexRes::Found(indexes);
            }
        };
    }
    MultiIndexRes::NotFound(desired_index_orig - desired_index)
}

fn insert_task_to_task_tree(
    task_list: &mut Vec<Task>,
    task: Task,
    multi_index: &[usize],
) -> Option<()> {
    if multi_index.len() == 1 {
        if task_list.len() <=  multi_index[0] {
            return None;
        }
        task_list.insert(multi_index[0], task);
        return Some(());
    }

    match multi_index.split_first() {
        None => task_list.push(task),
        Some((first_index, rest_of_indexes)) => match task_list.get_mut(*first_index) {
            Some(subtask) => {
                insert_task_to_task_tree(&mut subtask.subtasks, task, rest_of_indexes);
            }
            None => {
                return None;
            }
        },
    }
    Some(())
}
