pub mod event;
use std::collections::HashMap;
use redis::Connection;
use sqlx::Type;
use crate::api::Event;
use crate::model::event::EventModel;
use prost::Message;

// pub trait Message {
//     fn update_struct(&mut self, data: Event) -> ();
// }

pub  trait  Model {
    fn delete<E>(&self) -> Result<(), E>;
    fn new(item: Event) -> Self;
}

pub trait DBModel:Model {
    async fn set(&self) -> Result<(), String>;
    async fn update<E>(&self) -> Result<(), E>;
    fn get<T:Message, E>(key: &str) -> Result<Option<T>, E>;
    fn select<T:Message,E,S>(filter: HashMap<&str, &str, S>) -> Result<Option<Vec<T>>, E>;
    fn delete<T, E>(id: T) -> Result<(), E>;
}