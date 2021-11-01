pub use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WindowElements {
    Search,
    TitleBar,
    Results,
    Data,
}

pub fn handle_input(app: &mut App, keys: &Arc<Mutex<Vec<String>>>) {
    if let Event::Key(event) = read().unwrap() {
        match event.code {
            KeyCode::Esc => app.element_selected = false,
            _ => {
                if app.element_selected {
                    match app.highlighted_element {
                        WindowElements::Search => match event.code {
                            KeyCode::Char(ch) => {
                                app.input.push(ch);
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            KeyCode::Enter => {
                                app.state = AppState::DisplayResults;

                                app.element_selected = false;
                            }
                            _ => (),
                        },
                        WindowElements::TitleBar => (),
                        WindowElements::Results => match event.code {
                            KeyCode::Char('j') => {
                                let mut i = 0;

                                if let Some(x) = app.result_state.selected() {
                                    if app.num_showed_elements > 0 {
                                        if x == (app.num_showed_elements - 1) {
                                            if app.num_showed_elements < app.page_size {
                                                i = x;
                                            } else {
                                                app.result_page += 1;
                                            }
                                        } else {
                                            i = x + 1;
                                        }
                                    }
                                }

                                app.result_state.select(Some(i))
                            }
                            KeyCode::Char('k') => {
                                let mut i = 0;

                                if let Some(x) = app.result_state.selected() {
                                    if x == 0 {
                                        app.result_page = app.result_page.saturating_sub(1);
                                        i = app.page_size.saturating_sub(1);
                                    } else {
                                        i = x - 1;
                                    }
                                }

                                app.result_state.select(Some(i))
                            }
                            KeyCode::Enter => {
                                if let Some(x) = app.result_state.selected() {
                                    app.highlighted_element = WindowElements::Data;
                                    app.element_selected = false;

                                    let keys_cloned = keys.lock().unwrap().clone();
                                    let key = keys_cloned.get(x);

                                    if let Some(k) = key {
                                        app.selected_key = k.clone();
                                        app.state = AppState::DisplayDetails;
                                    }
                                }
                            }
                            _ => (),
                        },
                        WindowElements::Data => (),
                    }
                } else {
                    match event.code {
                        KeyCode::Char('q') => {
                            app.state = AppState::Exit;
                        }
                        KeyCode::Char('/') => {
                            app.highlighted_element = WindowElements::Search;
                            app.element_selected = true;
                        }
                        KeyCode::Char(ch) => {
                            transition(ch, app);
                            app.element_selected = false;
                        }
                        KeyCode::Enter => {
                            app.element_selected = true;

                            if app.highlighted_element == WindowElements::Results {
                                if let None = app.result_state.selected() {
                                    app.result_state.select(Some(0))
                                }
                            } else if app.highlighted_element == WindowElements::Search {
                                app.state = AppState::SearchSelected
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn transition(ch: char, app: &mut App) {
    match app.highlighted_element {
        WindowElements::Search => match ch {
            'h' => app.highlighted_element = WindowElements::TitleBar,
            'l' => app.highlighted_element = WindowElements::TitleBar,
            'j' => app.highlighted_element = WindowElements::Results,
            'k' => app.highlighted_element = WindowElements::Results,
            _ => (),
        },
        WindowElements::TitleBar => match ch {
            'h' => app.highlighted_element = WindowElements::Search,
            'l' => app.highlighted_element = WindowElements::Search,
            'j' => app.highlighted_element = WindowElements::Data,
            'k' => app.highlighted_element = WindowElements::Data,
            _ => (),
        },
        WindowElements::Results => match ch {
            'h' => app.highlighted_element = WindowElements::Data,
            'l' => app.highlighted_element = WindowElements::Data,
            'j' => app.highlighted_element = WindowElements::Search,
            'k' => app.highlighted_element = WindowElements::Search,
            _ => (),
        },
        WindowElements::Data => match ch {
            'h' => app.highlighted_element = WindowElements::Results,
            'l' => app.highlighted_element = WindowElements::Results,
            'j' => app.highlighted_element = WindowElements::TitleBar,
            'k' => app.highlighted_element = WindowElements::TitleBar,
            _ => (),
        },
    }
}
