use crossterm::style::Stylize;
use tbd::color::get_title_color;

use crate::tui::{Tui, tui_mode::TuiMode};

impl Tui {
    pub fn render_ui(&self) -> String {
        let mut lines: Vec<String> = vec![];

        for task in &self.task_file.tasks {
            let color = get_title_color(&task.title);
            let colored_text = task.render_screen(color);
            lines.push(colored_text);
        }
        let all_tasks = lines.join("\n");
        let mut pretty: Vec<String> = vec![];
        for (i, line) in all_tasks.lines().enumerate() {
            let mut pretty_line = String::from("");
            let cursor = ">>> ".white().to_string();
            pretty_line += if self.cursor == i { &cursor } else { "    " };
            pretty_line += line;
            pretty.push(pretty_line.to_string());
        }

        let mut full = self.render_header();
        full += pretty.join("\n").as_str();
        full += self.render_footer().as_str();
        full
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
            TuiMode::Edit => "Edit",
            TuiMode::Move => "Move",

            TuiMode::Normal => "Normal",
        };
        let message = if let Some(message) = &self.message {
            message
        } else {
            ""
        };
        let hint = &self.get_hint().dark_grey().to_string();
        format!(
            "\n\n  {mode_label} {mode}   {message}\n{hint}",
            mode_label = "mode:".dark_grey()
        )
    }
}
