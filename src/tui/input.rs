use crate::tui::Tui;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tbd::task_file::{IndexRes, MultiIndexRes, index_to_multi_index, multi_index_to_index};
use tbd::{Task, TaskState};

pub enum TuiInputMode {
    Normal,
    Edit,
    Move,
}

impl Tui {
    pub fn handle_input(&mut self, key_event: KeyEvent) -> Option<()> {
        match self.mode {
            TuiInputMode::Normal => {
                match key_event.code {
                    KeyCode::Char('J') | KeyCode::Down
                        if key_event.modifiers.contains(KeyModifiers::SHIFT) =>
                    {
                        let steps = (self.task_file.tasks_count() - self.cursor - 1).min(5);
                        if steps > 0 {
                            self.cursor += steps;
                        }
                    }
                    KeyCode::Char('K') | KeyCode::Up
                        if key_event.modifiers.contains(KeyModifiers::SHIFT) =>
                    {
                        let steps = (self.cursor).min(5);
                        if steps > 0 {
                            self.cursor -= steps;
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        if self.task_file.tasks_count() > self.cursor + 1 {
                            self.cursor += 1;
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::PageUp => {
                        if self.cursor > 0
                            && let MultiIndexRes::Found(mi) =
                                index_to_multi_index(&self.task_file.tasks, self.cursor)
                        {
                            if mi.len() == 1 {
                                if let IndexRes::Found(i) =
                                    multi_index_to_index(&self.task_file.tasks, &[mi[0] - 1])
                                {
                                    self.cursor = i;
                                }
                            } else {
                                if let IndexRes::Found(i) = multi_index_to_index(
                                    &self.task_file.tasks,
                                    &mi[0..mi.len() - 1],
                                ) {
                                    self.cursor = i;
                                }
                            }
                        }
                    }
                    KeyCode::PageDown => {
                        if let MultiIndexRes::Found(mi) =
                            index_to_multi_index(&self.task_file.tasks, self.cursor)
                        {
                            if mi.len() == 1 {
                                if let IndexRes::Found(i) =
                                    multi_index_to_index(&self.task_file.tasks, &[mi[0] + 1])
                                {
                                    self.cursor = i;
                                }
                            } else {
                                let mut level_down = mi.clone();
                                let second_last = level_down.len() - 2;
                                level_down[second_last] += 1;
                                if let IndexRes::Found(i) = multi_index_to_index(
                                    &self.task_file.tasks,
                                    &level_down[0..mi.len() - 1],
                                ) {
                                    self.cursor = i;
                                }
                            }
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
                    KeyCode::Char('S') => {
                        self.message = match self.task_file.save_file() {
                            Ok(_) => Some("file saved".to_owned()),
                            Err(_) => Some("failed to save file".to_string()),
                        }
                        //todo - return errors and display here
                    }
                    KeyCode::Char('D') => {
                        self.task_file.remove_task(self.cursor);
                        if self.task_file.tasks_count() > 0 {
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Char('m') => {
                        self.mode = TuiInputMode::Move;
                    }
                    KeyCode::Char('i') => {
                        self.mode = TuiInputMode::Edit;
                    }
                    KeyCode::Char('a') => {
                        self.task_file.insert_task(Task::new(), Some(self.cursor));
                        if self.task_file.tasks_count() > 1 {
                            self.cursor += 1;
                        }
                        self.mode = TuiInputMode::Edit;
                    }
                    KeyCode::Char('f') => {
                        if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                            task.is_collapsed = !task.is_collapsed;
                        }
                    }
                    KeyCode::Char('x') => {
                        if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                            task.state = TaskState::Done;
                        }
                    }
                    KeyCode::Char('.') => {
                        if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                            task.state = TaskState::Started;
                        }
                    }
                    KeyCode::Char('-') => {
                        if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                            task.state = TaskState::Skipped;
                        }
                    }
                    KeyCode::Char('n') => {
                        if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                            task.state = TaskState::NonTask;
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            }
            TuiInputMode::Move => match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.task_file.move_task_down(self.cursor).is_some() {
                        self.cursor += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.task_file.move_task_up(self.cursor).is_some() {
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
                    self.mode = TuiInputMode::Normal;
                }
                _ => {
                    return None;
                }
            },
            TuiInputMode::Edit => match key_event.code {
                KeyCode::Char(c) => {
                    if let Some(task) = self.task_file.get_task_at_mut(self.cursor) {
                        task.title.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some(task) = self.task_file.get_task_at_mut(self.cursor)
                        && !task.title.is_empty()
                    {
                        task.title = task.title[..task.title.len() - 1].to_string();
                    }
                }
                KeyCode::Enter | KeyCode::Esc => {
                    self.mode = TuiInputMode::Normal;
                }
                _ => {
                    return None;
                }
            },
        }
        Some(())
    }

    pub fn get_hint(&self) -> &str {
        match self.mode {
            TuiInputMode::Normal => {
                "
  ↓/↑/j/k: move cursor   PgUp/PgDn: move faster
  ←/→/h/l: indentation       space: cycle states
  i: edit      a: add       f: toggle fold
  S: save      Q: quit      D: delete
  [x] [.] [-] : set state n: toggle note"
  
            }
            TuiInputMode::Edit => {
                "
  esc/enter: confirm"
            }
            TuiInputMode::Move => {
                "
  ↓/↑/j/k:   move task  ←/→/h/l: indentation
  esc/enter: confirm"
            }
        }
    }
}
