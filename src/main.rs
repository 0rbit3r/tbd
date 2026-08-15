use std::env;
use std::fs;
use tbd::task_file::TaskFile;

mod tui;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Provide a .tbd file to open or create");
        return; //todo return proper err code
    }

    let file_exists;
    match fs::exists(&args[1]) {
        Err(_) => file_exists = false,
        Ok(bool) => match bool {
            false => {
                file_exists = false;
                eprintln!("error, something with symlinks, I guess");
            }
            true => file_exists = true,
        },
    }
    if file_exists {
        match TaskFile::from_file(&args[1]) {
            //todo - parser
            Err(e) => eprintln!("{e}"),
            Ok(task_file) => {
                let _ = tui::run(task_file);
            }
        }
    } else {
        let _ = tui::run(TaskFile::new(&args[1]));
    }
}
