use tbd::task_file::TaskFile;

fn main() {
    println!("==========================");
    println!("|       to be done       |");
    println!("==========================");
    println!();

    let task_file = TaskFile::from_file("tasks.tbd")
        .expect("failed to load file"); 

    for task in task_file.tasks {
        println!("{}", task.render());
    }

    println!();
}
