use sqlx::prelude::*;
use std::env::var as envar;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySql, MySqlPool, Pool};

pub async fn connect() -> Result<Pool<MySql>, Error> {
    let url =  get_conn_url();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await;

    pool
}

// pub fn connect_base() -> Result<Conn> {
//     let url = get_conn_url();
//     Conn::new(url.as_str())
// }

fn get_conn_url() -> String {
    let url = format!("mysql://{}:{}@{}:{}/{}",
                      envar("MYSQL_USER").unwrap(),
                      envar("MYSQL_PASSWORD").unwrap(),
                      envar("MYSQL_HOST").unwrap(),
                      envar("MYSQL_PORT").unwrap(),
                      envar("MYSQL_DB").unwrap());
    url
}