pub mod event;
use std::collections::HashMap;
use redis::Connection;
use crate::api::Event;


pub trait Message {
    //fn update_struct(&mut self, data: Event) -> ();
}

pub trait Model {
    fn delete<E>(&self) -> Result<(), E>;
}

pub trait DBModel:Model {
    async fn set<E>(&self, connection: Connection) -> Result<(), E>;
    async fn update<E>(&self, connection: Connection) -> Result<(), E>;
    fn get<T:Message, E>(key: &str, connection: Connection) -> Result<Option<T>, E>;
    fn select<T:Message,E,S>(filter: HashMap<&str, &str, S>, connection: Connection) -> Result<Option<Vec<T>>, E>;
    fn delete<T, E>(id: T, connection: Connection) -> Result<(), E>;
}