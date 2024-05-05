extern crate redis;

use std::env;
use redis::Client;
use redis::aio::MultiplexedConnection;
use std::env::var as envar;
use std::path::PathBuf;

pub async fn connect() -> redis::RedisResult<MultiplexedConnection> {
    let mut client:Client;

    if envar("REDIS_PASSWORD").unwrap() == "none" {
        client = redis::Client::open(format!("redis://{}:{}/{}",
                                                 envar("REDIS_HOST").unwrap(),
                                                 envar("REDIS_PORT").unwrap(),
                                                 envar("REDIS_DB").unwrap())).unwrap();
    } else {
        client = redis::Client::open(format!("redis://{}:{}@{}:{}/{}",
                                                 envar("REDIS_USER").unwrap(),
                                                 envar("REDIS_PASSWORD").unwrap(),
                                                 envar("REDIS_HOST").unwrap(),
                                                 envar("REDIS_PORT").unwrap(),
                                                 envar("REDIS_DB").unwrap())).unwrap();
    }

    let multi_con = client.get_multiplexed_tokio_connection().await.unwrap();

    //let con = client.get_connection().unwrap();

    Ok(multi_con)
}