use diesel::connection::{Connection};
use std::env::var as envar;
use diesel::prelude::{*,MysqlConnection};

pub fn connect() -> Result<MysqlConnection, ConnectionError> {
    let url =  get_conn_url();

    let connection = MysqlConnection::establish(&url.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", &url.as_str()));

    Ok(connection)
}

fn get_conn_url() -> String {
  format!("mysql://{}:{}@{}:{}/{}",
                      envar("MYSQL_USER").unwrap(),
                      envar("MYSQL_PASSWORD").unwrap(),
                      envar("MYSQL_HOST").unwrap(),
                      envar("MYSQL_PORT").unwrap(),
                      envar("MYSQL_DB").unwrap())
}