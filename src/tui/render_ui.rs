use crate::tui::{Tui, tui_mode::TuiMode};

impl Tui {
    pub fn render_ui(&self) -> String {
        let mut lines: Vec<String> = vec![];

        for task in &self.task_file.tasks {
            lines.push(task.render_screen())
        }
        let all_tasks = lines.join("\n");
        let mut pretty: Vec<String> = vec![];
        for (i, line) in all_tasks.lines().enumerate() {
            let mut pretty_line = String::from("");
            pretty_line += if self.cursor == i { ">>> " } else { "    " };
            pretty_line += line;
            pretty.push(pretty_line.to_string());
        }

        let mut full = self.render_header();
        full += pretty.join("\n").as_str();
        full += self.render_footer().as_str();
        full
    }

    fn render_header(&self) -> String {
        let title = match &self.task_file.path {
            None => "    new file",
            Some(f) => f
                .split("/")
                .last()
                .map(|s| s.trim_end_matches(".tbd"))
                .unwrap_or("    no file"),
        };
        format!(
            "    {title}\n----{} to be done -\n\n",
            "-".repeat(title.len() + 4)
        )
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
        format!("\n\n  mode: {mode}   {message}\n",)
    }
}
