use crate::prelude::*;

pub struct Config {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub db: String,
}

impl Config {
    pub fn new(host: &str, port: &str, username: &str, password: &str, db: &str) -> Self {
        Self {
            host: String::from(host),
            port: String::from(port),
            username: String::from(username),
            password: String::from(password),
            db: String::from(db),
        }
    }

    pub fn from_command_line() -> Self {
        let args = clap::App::new("Redis TUI (terminal UI)")
            .version("1.0")
            .author("Arba")
            .about("Provides a simple terminal UI to see your redis keys")
            .arg(
                Arg::with_name("host")
                    .long("host")
                    .default_value("127.0.0.1")
                    .help("Custom host for your redis cluster."),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .default_value("6379")
                    .help("Custom port for your redis cluster."),
            )
            .arg(
                Arg::with_name("username")
                    .short("u")
                    .long("username")
                    .default_value("")
                    .help("Custom username for your redis cluster."),
            )
            .arg(
                Arg::with_name("password")
                    .long("password")
                    .default_value("")
                    .help("Custom password for your redis cluster."),
            )
            .arg(
                Arg::with_name("db")
                    .long("db")
                    .default_value("")
                    .help("Custom db for your redis cluster."),
            )
            .get_matches();

        let host = args.value_of("host").unwrap();
        let port = args.value_of("port").unwrap();
        let username = args.value_of("username").unwrap();
        let password = args.value_of("password").unwrap();
        let db = args.value_of("db").unwrap();

        Config::new(host, port, username, password, db)
    }
}
