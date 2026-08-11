use std::env;
use tbd::task_file::TaskFile;

mod tui;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Provide a .tbd file to open");
        return; //todo return proper err code
    }

    match TaskFile::from_file(&args[1]) {
        //todo - parser
        Err(e) => eprintln!("{e}"),
        Ok(task_file) => {
            let _ = tui::run(task_file);
        }
    }

    println!();
}
