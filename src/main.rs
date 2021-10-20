mod app;
mod config;
mod handle_input;
mod redis;
mod screens;

mod prelude {
    pub use std::fmt::Display;
    pub use std::io::{self, Stdout, Write};

    pub use clap::{self, Arg};
    pub use crossterm::event::{read, Event, KeyCode, KeyEvent};
    pub use crossterm::terminal;
    pub use tui::backend::{Backend, CrosstermBackend};
    pub use tui::layout::{Alignment, Constraint, Direction, Layout};
    pub use tui::style::*;
    pub use tui::text::{Span, Spans};
    pub use tui::widgets::canvas::*;
    pub use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
    pub use tui::Terminal;
    pub use tui::*;

    pub use crate::app::*;
    pub use crate::config::*;
    pub use crate::handle_input::*;
    pub use crate::redis::*;
    pub use crate::screens::*;
}

use std::process::exit;

use ::redis::Commands;
use prelude::*;

fn main2() {
    let config = Config::from_command_line();
    let url = redis::url_builder(
        config.host,
        config.port,
        config.username,
        config.password,
        config.db,
    );

    let redis_client = redis::RedisClient::new(url);

    create_dataset(&redis_client);

    let mut con = redis_client.new_connection();
    let results = con.scan::<String>().unwrap();
    println!("{:?}", results.skip(20).take(1).collect::<Vec<String>>());
}

fn create_dataset(redis_client: &RedisClient) {
    let mut con = redis_client.new_connection();

    for i in 100..150 {
        let _ = con
            .set::<String, String, String>(format!("test{}", i), format!("value{}", i))
            .unwrap();
    }
}

fn main() {
    let config = Config::from_command_line();
    let url = redis::url_builder(
        config.host,
        config.port,
        config.username,
        config.password,
        config.db,
    );

    let redis_client = redis::RedisClient::new(url);

    let mut app = App::new();
    let mut terminal = init_terminal();

    loop {
        match app.state {
            AppState::Exit => {
                terminal_cleanup(&mut terminal);
                exit(0)
            }

            AppState::Selecting => {
                let mut con = redis_client.new_connection();
                let results_iter = redis_client.scan::<String>(&mut con).unwrap();

                let paged_results = results_iter
                    .skip(app.result_page * app.page_size)
                    .take(app.page_size)
                    .collect::<Vec<String>>();

                terminal
                    .draw(|f| {
                        draw_confirmed_search_screen(f, &mut app, paged_results);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app);
            }

            AppState::SearchSelected => {
                let mut con = redis_client.new_connection();
                let results_iter = redis_client
                    .scan_match::<String>(&mut con, String::from(&app.input))
                    .unwrap();

                let paged_results = results_iter
                    .skip(app.result_page * app.page_size)
                    .take(app.page_size)
                    .collect::<Vec<String>>();

                terminal
                    .draw(|f| {
                        draw_confirmed_search_screen(f, &mut app, paged_results);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app);
            }

            AppState::DisplayResults => {
                let mut con = redis_client.new_connection();
                let results_iter = redis_client
                    .scan_match::<String>(&mut con, String::from(&app.input))
                    .unwrap();

                let paged_results = results_iter
                    .skip(app.result_page * app.page_size)
                    .take(app.page_size)
                    .collect::<Vec<String>>();

                terminal
                    .draw(|f| {
                        draw_confirmed_search_screen(f, &mut app, paged_results);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app);
            }

            AppState::DisplayDetails => {
                let mut con = redis_client.new_connection();
                let value = con.get(&app.selected_key).unwrap();

                let details = generate_display_text(value);

                terminal
                    .draw(|f| {
                        draw_display_details(f, &mut app, details);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app);
            }
        }
    }
}
