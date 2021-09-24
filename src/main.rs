mod app;
mod config;
mod handle_input;
mod redis_client;

mod prelude {
    pub use std::io::{self, Stdout, Write};

    pub use clap::{self, Arg};
    pub use crossterm::event::{read, Event, KeyCode, KeyEvent};
    pub use crossterm::terminal;
    pub use redis;
    pub use tui::backend::{Backend, CrosstermBackend};
    pub use tui::layout::{Alignment, Constraint, Direction, Layout};
    pub use tui::style::{Color, Style};
    pub use tui::widgets::{Block, Borders, Paragraph};
    pub use tui::Terminal;

    pub use crate::app::*;
    pub use crate::config::*;
    pub use crate::handle_input::*;
    pub use crate::redis_client::*;
}

use prelude::*;

fn main() {
    let config = Config::from_command_line();
    let url = redis_client::url_builder(
        config.host,
        config.port,
        config.username,
        config.password,
        config.db,
    );
    let client = redis::Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();

    let _: () = redis::cmd("SET")
        .arg("my_key")
        .arg("value1")
        .query(&mut con)
        .unwrap();

    let my_value: String = redis::cmd("GET").arg("my_key").query(&mut con).unwrap();

    println!("{}", my_value);

    let mut app = App::new();

    loop {
        let input_text = String::from(&app.input);
        app.terminal
            .draw(|f| {
                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(6), Constraint::Percentage(94)].as_ref())
                    .split(f.size());

                let search_and_tabs = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
                    .split(main_layout[0]);

                let result_preview_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                    .split(main_layout[1]);

                let paragraph = Paragraph::new(input_text)
                    .alignment(Alignment::Left)
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL).title("input"));

                let block2 = Block::default().title("Block2").borders(Borders::ALL);
                let block3 = Block::default().title("Block3").borders(Borders::ALL);
                let block4 = Block::default().title("Block4").borders(Borders::ALL);

                f.render_widget(paragraph, search_and_tabs[0]);
                f.render_widget(block2, search_and_tabs[1]);
                f.render_widget(block3, result_preview_layout[0]);
                f.render_widget(block4, result_preview_layout[1])
            })
            .unwrap();
    }
}
