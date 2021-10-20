pub use crate::prelude::*;

use redis::*;

#[derive(Clone)]
pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(url: String) -> Self {
        let client = redis::Client::open(url).unwrap();

        Self { client }
    }

    pub fn new_connection(&self) -> redis::Connection {
        self.client.get_connection().unwrap()
    }

    pub fn scan<'a, T: FromRedisValue>(&self, con: &'a mut Connection) -> RedisResult<Iter<'a, T>> {
        con.scan()
    }

    pub fn scan_match<'a, T: ToRedisArgs + FromRedisValue + Display>(
        &self,
        con: &'a mut Connection,
        pattern: T,
    ) -> RedisResult<Iter<'a, T>> {
        con.scan_match(format!("{}*", pattern))
    }
}

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
