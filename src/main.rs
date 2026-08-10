use tbd::simple_cli::run_simple_cli;
use tbd::task_file::TaskFile;

fn main() {
    match TaskFile::from_file("tasks.tbd") {
        Err(e) => eprintln!("{e}"),
        Ok(mut task_file) => {
            task_file.cursor = Some(0);
            run_simple_cli(task_file);
        }
    }

    println!();
}
