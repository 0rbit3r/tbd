use std::env;
use tbd::task_file::TaskFile;

use crate::task_file_selector::FindTaskFileRes;
use crate::task_file_selector::find_task_file_to_open;

mod task_file_selector;
mod tui;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "--version" || args[1] == "-v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.len() >= 2 && args[1] == "--help" {
        println!("tbd v{}", env!("CARGO_PKG_VERSION"));
        println!("usage:\n  tbd [path]");
        println!("    - where path can be:");
        println!("      - a .tbd file - opens the file (the extension can be omitted)");
        println!("      - a directory - either opens the first .tbd file found or creates tasks.tbd");
        println!("      - omitted - alias for `tbd .`");
        println!("\n  tbd --version | -v");
        println!("    - displays version");
        println!("\n  tbd --help");
        println!("    - displays this help\n");
        return;
    }

    if args.len() < 2 {
        args.push(".".to_string());
    }

    match find_task_file_to_open(&args[1]) {
        FindTaskFileRes::Found(file) => {
            if let Ok(task_file) = TaskFile::from_file(&file) {
                _ = tui::run(task_file);
            }
        }
        FindTaskFileRes::New(file) => {
            let _ = tui::run(TaskFile::new(&file));
        }
    }
}
