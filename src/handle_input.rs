pub use crate::prelude::*;

pub enum WindowElements {
    Search,
    TitleBar,
    Results,
    Data,
}

pub fn handle_input(app: &mut App) {
    if let Event::Key(event) = read().unwrap() {
        match event.code {
            KeyCode::Char('q') => app.terminal_cleanup(),
            KeyCode::Char(ch) => app.input.push(ch),
            KeyCode::Backspace => {
                app.input.pop();
            }
            _ => {}
        }
    }
}
