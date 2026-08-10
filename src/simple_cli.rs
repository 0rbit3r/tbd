use std::io;

use crate::task_file::TaskFile;

pub fn run_simple_cli(mut task_file: TaskFile) {
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("==========================");
        println!("|       to be done       |");
        println!("==========================");
        println!();

        println!("{}", task_file.render_screen());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("simple!");

        match input.trim() {
            "j" => {
                println!("alspdfka");
                task_file.cursor = task_file.cursor.map(|n| n + 1);
            }
            "k" => {
                task_file.cursor = task_file.cursor.map(|n| n - 1);
            }
            "l" => if let Some(c) = task_file.cursor {
                    task_file.indent_task(c);
            },
            "h" => if let Some(c) = task_file.cursor {
                    task_file.unindent_task(c);
            },
            _ => {
                println!("invalid option");
                io::stdin().read_line(&mut input).expect("simple!");
            }
        }
    }
}
