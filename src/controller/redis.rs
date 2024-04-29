extern crate redis;
use redis::Connection;

fn connect() -> redis::RedisResult<Connection> {
    let client = redis::Client::open(format!("redis://{}/", std::env::var("REDIS_HOST").unwrap()))?;
    let mut con = client.get_connection()?;

    Ok(con)
}