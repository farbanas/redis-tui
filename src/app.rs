use crate::prelude::*;

pub struct App {
    pub element_selected: bool,
    pub highlighted_element: WindowElements,
    pub input: String,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            element_selected: false,
            highlighted_element: WindowElements::Search,
            input: String::new(),
            terminal: init_terminal(),
        }
    }

    pub fn terminal_cleanup(&mut self) {
        terminal::disable_raw_mode().unwrap();
        self.terminal.clear().unwrap();
    }
}

fn init_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    terminal::enable_raw_mode().unwrap();

    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal
}
