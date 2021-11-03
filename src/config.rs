use crate::prelude::*;

pub struct Config {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub db: String,
    pub max_matches: i64,
}

impl Config {
    pub fn new(
        host: &str,
        port: &str,
        username: &str,
        password: &str,
        db: &str,
        max_matches: &str,
    ) -> Self {
        Self {
            host: String::from(host),
            port: String::from(port),
            username: String::from(username),
            password: String::from(password),
            db: String::from(db),
            max_matches: max_matches.parse::<i64>().unwrap(),
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
            .arg(
                Arg::with_name("max_matches")
                    .long("max_matches")
                    .default_value("200")
                    .help("The maximum number of matches for a search that this app will fetch."),
            )
            .get_matches();

        let host = args.value_of("host").unwrap();
        let port = args.value_of("port").unwrap();
        let username = args.value_of("username").unwrap();
        let password = args.value_of("password").unwrap();
        let db = args.value_of("db").unwrap();
        let max_matches = args.value_of("max_matches").unwrap();

        Config::new(host, port, username, password, db, max_matches)
    }
}
