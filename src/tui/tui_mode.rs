use crossterm::event::{KeyCode, KeyEvent};

use crate::{task::Task, task_state::TaskState, tui::Tui};

pub enum TuiMode {
    Normal,
    Edit,
    Move,
}

impl Tui {
    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match self.mode {
            TuiMode::Normal => {
                match key_event.code {
                    KeyCode::Char('j') | KeyCode::Down => {
                        //todo iterator for taskFile and length check here
                        self.cursor += 1;
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        self.task_file.indent_task(self.cursor);
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        if let Some(dropped_by) = self.task_file.unindent_task(self.cursor) {
                            self.cursor += dropped_by
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                            task.state = match task.state {
                                TaskState::Untouched => TaskState::Started,
                                TaskState::Started => TaskState::Done,
                                TaskState::Done => TaskState::Skipped,
                                _ => TaskState::Untouched,
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        self.message = match self.task_file.save_file() {
                            Ok(_) => Some("file saved".to_owned()),
                            Err(_) => Some("failed to save file".to_string()),
                        }
                        //todo - return errors and display here
                    }
                    KeyCode::Char('d') => {
                        self.task_file.remove_task(self.cursor);
                    }
                    KeyCode::Char('m') => {
                        self.mode = TuiMode::Move;
                    }
                    KeyCode::Char('i') => {
                        self.mode = TuiMode::Edit;
                    }
                    KeyCode::Char('a') => {
                        self.task_file.insert_task(Task::new(), Some(self.cursor));
                        self.cursor += 1;
                        self.mode = TuiMode::Edit;
                    }
                    _ => {}
                }
            }
            TuiMode::Move => match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    if let Some(_) = self.task_file.move_task_down(self.cursor) {
                        self.cursor += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if let Some(_) = self.task_file.move_task_up(self.cursor) {
                        self.cursor -= 1;
                    }
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    self.task_file.indent_task(self.cursor);
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    if let Some(dropped_by) = self.task_file.unindent_task(self.cursor) {
                        self.cursor += dropped_by
                    }
                }
                KeyCode::Enter | KeyCode::Esc => {
                    self.mode = TuiMode::Normal;
                }
                _ => (),
            },
            TuiMode::Edit => match key_event.code {
                KeyCode::Char(c) => {
                    if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                        task.title.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                        if !task.title.is_empty() {
                            task.title = task.title[..task.title.len() - 1].to_string();
                        }
                    }
                }
                KeyCode::Enter | KeyCode::Esc => {
                    self.mode = TuiMode::Normal;
                }
                _ => {}
            },
        }
    }
}
