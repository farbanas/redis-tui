use crate::prelude::*;

pub fn url_builder(
    host: String,
    port: String,
    username: String,
    password: String,
    db: String,
) -> String {
    if username.is_empty() {
        format!("redis://{}:{}/{}", host, port, db)
    } else {
        format!("redis://{}:{}@{}:{}/{}", username, password, host, port, db)
    }
}
