use crossterm::style::Stylize;
use crossterm::terminal;
use tbd::color::get_title_color;

use crate::tui::{Tui, input::TuiInputMode};

impl Tui {
    pub fn render_ui(&self) -> String {
        let mut lines: Vec<String> = vec![];

        for task in &self.task_file.tasks {
            let color = get_title_color(&task.title);
            let colored_text = task.render_screen(color);
            lines.push(colored_text);
        }
        let all_tasks = lines.join("\n");
        let mut pretty_tasks: Vec<String> = vec![];
        for (i, line) in all_tasks.lines().enumerate() {
            let mut pretty_line = String::from("");
            let cursor = ">>> ".white().to_string();
            pretty_line += if self.cursor == i { &cursor } else { "    " };
            pretty_line += line;
            pretty_tasks.push(pretty_line.to_string());
        }

        let header = self.render_header();
        let footer = self.render_footer();

        self.render_scrollable(header, &footer, &pretty_tasks)
    }

    fn render_header(&self) -> String {
        let title = &&self
            .task_file
            .path
            .split("/")
            .last()
            .map(|s| s.trim_end_matches(".tbd"))
            .unwrap_or("    no file");

        let mut string = String::new();
        string += &format!("        {title}\n");
        string += &"⎯⎯⎯⎯".dark_grey().to_string();
        string += &"⎯".dark_grey().to_string().repeat(title.len() + 4);
        string += &if self.task_file.saved {
            " to be done ".dark_grey().to_string()
        } else {
            " not saved! ".yellow().to_string()
        };
        string += &"⎯\n\n".dark_grey().to_string();
        string
    }

    fn render_footer(&self) -> String {
        let mode = match &self.mode {
            TuiInputMode::Edit => "Edit",
            TuiInputMode::Move => "Move",
            TuiInputMode::Normal => "Normal",
        };
        let message = if let Some(message) = &self.message {
            message
        } else {
            ""
        };
        let hint = if self.hint_displayed {
            &self.get_hint().dark_grey().to_string()
        } else {
            ""
        };
        format!(
            "\n\n  {mode_label} {mode}  {toggle_hint}  {message}\n{hint}",
            mode_label = "mode:".dark_grey(),
            toggle_hint = "?: toggle hint".dark_grey()
        )
    }

    fn render_scrollable(&self, header: String, footer: &str, tasks: &Vec<String>) -> String {
        let header_lines = header.lines().count();
        let footer_lines = footer.lines().count();
        let (_term_width, term_height) = if let Ok(term_size) = terminal::size() {
            (term_size.0.into(), term_size.1.into())
        } else {
            (60 as usize, 60 as usize)
        };

        let mut full = header;

        if header_lines + footer_lines + tasks.len() > term_height.into() {
            let available_space: usize = term_height - footer_lines - header_lines;
            let lower_bound = if self.cursor < tasks.len() - available_space / 2 {
                self.cursor - (available_space / 2).min(self.cursor)
            } else {
                tasks.len() - available_space
            };
            let upper_bound = (lower_bound + available_space).min(tasks.len());
            full += tasks[lower_bound..upper_bound].join("\n").as_str();
        } else {
            full += tasks.join("\n").as_str();
        }

        full += self.render_footer().as_str();
        full
    }
}
