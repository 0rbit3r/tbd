use crate::task::Task;
use crate::task_state::TaskState;

/// This function will parse the string into provided Tasks vector.
/// In case of syntax errors, affected lines will be added as Malformed tasks
pub fn parse_line_and_add_to_task_list(line: &str, tasks: &mut Vec<Task>) {
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
        TaskState::NonTask,
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
                        None => add_task(tasks, line, TaskState::NonTask),
                    };
                }
                None => add_task(tasks, line, TaskState::NonTask),
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::{task::Task, task_file::TaskFile, task_state::TaskState};

    fn get_shallow_tasks_str() -> &'static str {
        "[ ] Untouched
[.] Started
[-] Skipped
[x] Done
gibberish"
    }

    fn get_nested_tasks_str() -> &'static str {
        "[ ] Untouched
    [ ] Untouched nested
    subgibberish
[.] Started
    [.] Substarted 1
    [.] Substarted 2
[-] Skipped
    [-] Subskipped 1
        [-] Subsubskipped
    [-] Subskipped 2
[x] Done
        [ ] Too far gone
gibberish"
    }

    #[test]
    fn parse_shallow() {
        let task_file =
            TaskFile::from_string(get_shallow_tasks_str()).expect("This content is parseable");
        let expected = vec![
            Task {
                state: TaskState::Untouched,
                title: "Untouched".to_string(),
                subtasks: vec![],
            },
            Task {
                state: TaskState::Started,
                title: "Started".to_string(),
                subtasks: vec![],
            },
            Task {
                state: TaskState::Skipped,
                title: "Skipped".to_string(),
                subtasks: vec![],
            },
            Task {
                state: TaskState::Done,
                title: "Done".to_string(),
                subtasks: vec![],
            },
            Task {
                state: TaskState::NonTask,
                title: "gibberish".to_string(),
                subtasks: vec![],
            },
        ];
        for i in 0..5 {
            assert_eq!(expected[i], task_file.tasks[i]);
        }
    }

    #[test]
    fn parse_nested() {
        let task_file =
            TaskFile::from_string(get_nested_tasks_str()).expect("This content is parseable");
        let expected = vec![
            Task {
                state: TaskState::Untouched,
                title: "Untouched".to_string(),
                subtasks: vec![
                    Task {
                        state: TaskState::Untouched,
                        title: "Untouched nested".to_string(),
                        subtasks: vec![],
                    },
                    Task {
                        state: TaskState::NonTask,
                        title: "subgibberish".to_string(),
                        subtasks: vec![],
                    },
                ],
            },
            Task {
                state: TaskState::Started,
                title: "Started".to_string(),
                subtasks: vec![
                    Task {
                        state: TaskState::Started,
                        title: "Substarted 1".to_string(),
                        subtasks: vec![],
                    },
                    Task {
                        state: TaskState::Started,
                        title: "Substarted 2".to_string(),
                        subtasks: vec![],
                    },
                ],
            },
            Task {
                state: TaskState::Skipped,
                title: "Skipped".to_string(),
                subtasks: vec![
                    Task {
                        state: TaskState::Skipped,
                        title: "Subskipped 1".to_string(),
                        subtasks: vec![Task {
                            state: TaskState::Skipped,
                            title: "Subsubskipped".to_string(),
                            subtasks: vec![],
                        }],
                    },
                    Task {
                        state: TaskState::Skipped,
                        title: "Subskipped 2".to_string(),
                        subtasks: vec![],
                    },
                ],
            },
            Task {
                state: TaskState::Done,
                title: "Done".to_string(),
                subtasks: vec![Task {
                    state: TaskState::NonTask,
                    title: "    [ ] Too far gone".to_string(),
                    subtasks: vec![],
                }],
            },
            Task {
                state: TaskState::NonTask,
                title: "gibberish".to_string(),
                subtasks: vec![],
            },
        ];
        for i in 0..5 {
            assert_eq!(expected[i], task_file.tasks[i]);
        }
    }

    #[test]
    fn parse_and_render_mostly_equal() {
        let task_file =
            TaskFile::from_string(get_nested_tasks_str()).expect("content is parseable");
        let rendered = task_file.render_file();
        assert_eq!(get_nested_tasks_str(), rendered)
    }
}
