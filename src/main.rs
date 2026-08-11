use tbd::task_file::TaskFile;
use tbd::tui::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match TaskFile::from_file(&args[1]) { //todo - parser
        Err(e) => eprintln!("{e}"),
        Ok(task_file) => {
            let _ = run(task_file);
        }
    }

    println!();
}
