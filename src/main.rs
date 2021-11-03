mod app;
mod config;
mod handle_input;
mod redis;
mod screens;

mod prelude {
    pub use std::fmt::Display;
    pub use std::io::{self, Stdout, Write};
    pub use std::process::exit;
    pub use std::sync::atomic::Ordering::SeqCst;
    pub use std::sync::mpsc;
    pub use std::sync::mpsc::{Receiver, Sender};
    pub use std::sync::{atomic::AtomicBool, mpsc::TryRecvError};
    pub use std::sync::{Arc, Mutex};
    pub use std::thread;
    pub use std::time::Duration;

    pub use clap::{self, Arg};
    pub use crossterm::event::{read, Event, KeyCode, KeyEvent};
    pub use crossterm::terminal;
    pub use rand::*;
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

use ::redis::Commands;
use prelude::*;

fn create_dataset(redis_client: &RedisClient) {
    let mut con = redis_client.new_connection();

    let mut rng = rand::thread_rng();

    for i in 0..100 {
        let j: i32 = rng.gen();
        let _ = con
            .set::<String, String, String>(format!("test{}", j), format!("value{}", j))
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

    let mut con = redis_client.new_connection();
    let results_iter = redis_client.scan::<String>(&mut con).unwrap();

    let paged_results = results_iter
        .skip(app.result_page * app.page_size)
        .take(app.page_size)
        .collect::<Vec<String>>();

    let keys: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(paged_results));

    let (producer, consumer): (Sender<String>, Receiver<String>) = mpsc::channel();

    let keys_cloned = Arc::clone(&keys);
    thread::spawn(move || loop {
        match consumer.try_recv() {
            Ok(key) => {
                keys_cloned.lock().unwrap().push(key);
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    });

    let mut switch = true;
    let stop_searching_1: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let stop_searching_2: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    loop {
        match app.state {
            AppState::Exit => {
                terminal_cleanup(&mut terminal);
                exit(0)
            }

            AppState::Selecting => {
                terminal
                    .draw(|f| {
                        draw_selecting_screen(f, &mut app, &keys);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app, &keys);
            }

            AppState::SearchSelected => {
                terminal
                    .draw(|f| {
                        draw_selecting_screen(f, &mut app, &keys);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app, &keys);
            }

            AppState::DisplayResults => {
                let stop_searching_clone = if switch {
                    stop_searching_2.store(true, SeqCst);
                    stop_searching_2.store(false, SeqCst);
                    stop_searching_1.clone()
                } else {
                    stop_searching_1.store(true, SeqCst);
                    stop_searching_2.store(false, SeqCst);
                    stop_searching_2.clone()
                };
                switch = !switch;

                let new_redis_client = redis_client.clone();
                let pattern = app.input.clone();
                let new_producer = producer.clone();
                let max_matches = config.max_matches;

                keys.lock().unwrap().clear();

                thread::spawn(move || {
                    let mut con = new_redis_client.new_connection();

                    let mut keys = new_redis_client
                        .scan_match::<String>(&mut con, pattern)
                        .unwrap();

                    let mut current_results = 0;
                    loop {
                        if stop_searching_clone.load(SeqCst) {
                            break;
                        }

                        if let Some(k) = keys.next() {
                            let send_res = new_producer.send(k);
                            if let Err(e) = send_res {
                                println!("{}", e);
                                break;
                            }

                            current_results += 1;
                            if current_results == max_matches {
                                break;
                            }
                        }
                    }

                    drop(new_producer);
                });

                app.state = AppState::Selecting
            }

            AppState::DisplayDetails => {
                let mut con = redis_client.new_connection();
                let value = con.get(&app.selected_key).unwrap();

                let details = generate_display_text(value);

                terminal
                    .draw(|f| {
                        draw_display_details(f, &mut app, &keys, details);
                    })
                    .unwrap();

                handle_input::handle_input(&mut app, &keys);
            }
        }
    }
}
