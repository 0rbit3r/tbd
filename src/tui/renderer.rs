struct Renderer {
    last_screen: String,
}

impl Renderer {
    pub fn render(&mut self, screen: String) {
        let last_lines = &self.last_screen.lines();
        for line in screen.lines() {}
    }
}
