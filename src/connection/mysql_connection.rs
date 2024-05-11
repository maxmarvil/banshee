use mysql::*;
use mysql::prelude::*;
use std::env::var as envar;

pub fn connect() -> Result<PooledConn> {
    let url =  get_conn_url();
    let pool = Pool::new(url.as_str())?;
    let conn = pool.get_conn()?;

    Ok(conn)
}

pub fn connect_base() -> Result<Conn> {
    let url = get_conn_url();
    Conn::new(url.as_str())
}

fn get_conn_url() -> String {
    let url = format!("mysql://{}:{}@{}:{}/{}",
                      envar("MYSQL_USER").unwrap(),
                      envar("MYSQL_PASSWORD").unwrap(),
                      envar("MYSQL_HOST").unwrap(),
                      envar("MYSQL_PORT").unwrap(),
                      envar("MYSQL_DB").unwrap());
    url
}