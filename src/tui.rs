mod input;
mod raw_mode_guard;
mod render_ui;

use crate::tui::input::TuiInputMode;
use crate::tui::raw_mode_guard::RawModeGuard;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::io::Write;
use tbd::TaskFile;

pub fn run(task_file: TaskFile) -> io::Result<()> {
    let _raw_mode_guard = RawModeGuard::new()?;

    let mut tui = Tui::from_task_file(task_file);

    loop {
        let cursor_reset = format!("{esc}[1;1H", esc = 27 as char);
        let clear_line = format!("{esc}[0K", esc = 27 as char);
        let clear_to_end = format!("\n{esc}[0J", esc = 27 as char);

        let rendered_lines = tui.render_ui();
        tui.message = None;
        let mut rendered_lines = rendered_lines.lines().collect::<Vec<_>>();
        rendered_lines.push(&clear_to_end);
        let rendered_lines = rendered_lines.join(&format!("\r\n{clear_line}"));

        print!("{cursor_reset}{clear_line}{rendered_lines}");
        io::stdout().flush()?;

        if let Event::Key(key_event) = event::read()?
            && tui.handle_input(key_event).is_none()
        {
            match key_event.code {
                KeyCode::Char('?') => tui.hint_displayed = !tui.hint_displayed,
                KeyCode::Char('Q') => break,
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

struct Tui {
    task_file: TaskFile,
    cursor: usize,
    mode: TuiInputMode,
    message: Option<String>,
    hint_displayed: bool,
}

impl Tui {
    pub fn from_task_file(task_file: TaskFile) -> Tui {
        Tui {
            task_file,
            cursor: 0,
            mode: TuiInputMode::Normal,
            message: None,
            hint_displayed: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tui;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use tbd::TaskFile;

    const CONTENT: &str = "[ ] first
[.] second
    [x] second.1
    [-] second.2 ...
        [-] second.2.1
        gibberish
    non-task
    [x] second.3
[.] third
    [ ] third.1";

    #[test]
    fn move_cursor() {
        let task_file = TaskFile::from_string("path", CONTENT).unwrap();
        let mut tui = Tui::from_task_file(task_file);
        tui.handle_input(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()));
        assert_eq!(0, tui.cursor);
        for i in [1, 2, 3, 4, 5, 6, 7, 7] {
            tui.handle_input(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));
            assert_eq!(i, tui.cursor);
        }
    }
    #[test]
    fn jump_cursor() {
        let task_file = TaskFile::from_string("path", CONTENT).unwrap();
        let mut tui = Tui::from_task_file(task_file);
        tui.handle_input(KeyEvent::new(KeyCode::PageUp, KeyModifiers::empty()));
        assert_eq!(0, tui.cursor);
        for i in [1, 6, 6] {
            tui.handle_input(KeyEvent::new(KeyCode::PageDown, KeyModifiers::empty()));
            assert_eq!(i, tui.cursor);
        }
    }
    #[test]
    fn move_task() {
        let task_file = TaskFile::from_string("path", CONTENT).unwrap();
        let mut tui = Tui::from_task_file(task_file);
        let input_sequence = [
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Char('m'),
            KeyCode::Up,
            KeyCode::Up,
            KeyCode::Up,
            KeyCode::Enter,
        ];
        for key in input_sequence {
            tui.handle_input(KeyEvent::new(key, KeyModifiers::empty()));
            println!("\n----------\n{}", tui.task_file.render_file());
        }
        assert_eq!("second.3", tui.task_file.get_task_at(2).unwrap().title)
    }
}
