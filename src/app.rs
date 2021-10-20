use crate::prelude::*;
use redis::FromRedisValue;
use tui::widgets::ListState;

pub struct App {
    pub result_page: usize,
    pub page_size: usize,
    pub state: AppState,
    pub highlighted_element: WindowElements,
    pub element_selected: bool,
    pub input: String,
    pub result_state: ListState,
    pub num_showed_elements: usize,
    pub keys: Vec<String>,
    pub selected_key: String,
}

pub enum AppState {
    Selecting,
    SearchSelected,
    DisplayResults,
    DisplayDetails,
    Exit,
}

impl App {
    pub fn new() -> Self {
        Self {
            result_page: 0,
            page_size: 20,
            state: AppState::Selecting,
            highlighted_element: WindowElements::Search,
            element_selected: false,
            input: String::new(),
            result_state: ListState::default(),
            num_showed_elements: 0,
            keys: vec![],
            selected_key: String::new(),
        }
    }

    pub fn set_num_showed_elements(&mut self, num_elements: usize) {
        self.num_showed_elements = num_elements;
    }

    pub fn set_keys(&mut self, keys: &Vec<String>) {
        self.keys = keys.clone();
    }

    pub fn draw_results<T: FromRedisValue + Display>(&self, results: Vec<String>) -> List {
        let list_items: Vec<ListItem> = results
            .into_iter()
            .map(|key| ListItem::new(format!("{}", key)))
            .collect();

        List::new(list_items)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
    }
}

pub fn init_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    terminal::enable_raw_mode().unwrap();

    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal
}

pub fn terminal_cleanup(term: &mut Terminal<CrosstermBackend<Stdout>>) {
    term.clear().unwrap();
    term.show_cursor().unwrap();
    terminal::disable_raw_mode().unwrap();
}
