mod raw_mode_guard;
mod render_ui;
mod tui_mode;

use crate::tui::raw_mode_guard::RawModeGuard;
use crate::tui::tui_mode::TuiMode;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::io::Write;
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
        let clear_to_end = format!("\n{esc}[0J", esc = 27 as char);

        let rendered_lines = tui.render_ui();
        tui.message = None;
        let mut rendered_lines = rendered_lines.lines().collect::<Vec<_>>();
        rendered_lines.push(&clear_to_end);
        let rendered_lines = rendered_lines.join(&format!("\r\n{clear_line}"));

        print!("{cursor_reset}{rendered_lines}");
        io::stdout().flush()?;

        if let Event::Key(key_event) = event::read()? {
            if tui.handle_input(key_event).is_none(){
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => break,
                _ => {}
            }}
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
