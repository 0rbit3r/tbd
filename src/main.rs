use tbd::task::Task;
use tbd::task_file::TaskFile;
use tbd::task_state::TaskState;

fn main() {
    println!("==========================");
    println!("|       to be done       |");
    println!("==========================");
    println!();

    match TaskFile::from_file("tasks.tbd") {
        Ok(mut task_file) => {
            task_file.insert_task(
                Task {
                    title: "!!! NEW_TASK !!!".to_string(),
                    state: TaskState::Untouched,
                    subtasks: vec![],
                },
                Some(15),
            );

            for task in &task_file.tasks {
                println!("{}", task.render(false));
            }
            // match task_file.save_as("output.tbd") {
            //     Err(e) => {eprintln!("{e}")}
            //     _ => {}
            // }
        }
        Err(e) => eprintln!("{e}"),
    }

    println!();
}
