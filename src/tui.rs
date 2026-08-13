mod raw_mode_guard;
mod render_ui;
mod tui_mode;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::tui::raw_mode_guard::RawModeGuard;
use crate::tui::tui_mode::TuiMode;
use std::io;
use tbd::TaskFile;

pub fn run(task_file: TaskFile) -> io::Result<()> {
    let _raw_mode_guard = RawModeGuard::new()?;

    let mut tui = Tui {
        cursor: 0,
        mode: TuiMode::Normal,
        task_file,
        message: None,
    };

    loop {
        let cursor_reset = format!("{esc}[1;1H", esc = 27 as char);
        let clear_line = format!("{esc}[0K", esc = 27 as char);

        let rendered_lines = tui
            .render_ui()
            .lines()
            .collect::<Vec<_>>()
            .join(&format!("{clear_line}\r\n"));
        print!("{cursor_reset}{rendered_lines}");
        //            print!("{esc}[0J", esc = 27 as char);

        if let Event::Key(key_event) = event::read()? {
            tui.handle_input(key_event);
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => break,
                _ => {}
            }
        }
    }

    Ok(())
}

struct Tui {
    task_file: TaskFile,
    cursor: usize,
    mode: TuiMode,
    message: Option<String>,
}
