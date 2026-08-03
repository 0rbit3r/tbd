use tbd::model::{Task, TaskState};

fn main() {
    let tasks = vec![
        Task {
            id: 9,
            title: "task renderer".to_string(),
            state: TaskState::Done,
            subtasks: vec![
                Task {
                    id: 10,
                    title: "render method".to_string(),
                    state: TaskState::Done,
                    subtasks: vec![],
                },
                Task {
                    id: 11,
                    title: "subtasks support".to_string(),
                    state: TaskState::Done,
                    subtasks: vec![],
                },
            ],
        },
        Task {
            id: 8,
            title: "file parser and saver".to_string(),
            state: TaskState::Started,
            subtasks: vec![
                Task {
                    id: 5,
                    title: "file format".to_string(),
                    state: TaskState::Untouched,
                    subtasks: vec![],
                },
                Task {
                    id: 6,
                    title: "parser".to_string(),
                    state: TaskState::Untouched,
                    subtasks: vec![],
                },
                Task {
                    id: 7,
                    title: "saver".to_string(),
                    state: TaskState::Untouched,
                    subtasks: vec![],
                },
            ],
        },
        Task {
            id: 1,
            title: "api get tasks".to_string(),
            state: TaskState::Untouched,
            subtasks: vec![],
        },
        Task {
            id: 2,
            title: "api save tasks".to_string(),
            state: TaskState::Untouched,
            subtasks: vec![],
        },
        Task {
            id: 3,
            title: "api create task".to_string(),
            state: TaskState::Untouched,
            subtasks: vec![],
        },
        Task {
            id: 4,
            title: "simple cli".to_string(),
            state: TaskState::Untouched,
            subtasks: vec![],
        },
    ];

    println!("==========================");
    println!("|       to be done       |");
    println!("==========================");
    println!();

    for task in tasks {
        println!("{}", task.render());
    }

    println!();
}
