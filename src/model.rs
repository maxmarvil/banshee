pub mod event;
use std::collections::HashMap;
use sqlx::Type;
use crate::api::Event;
use prost::Message;
use crate::model::event::EventModel;


pub  trait  Model {
    fn delete_by_id<T, E>(id: T) -> Result<(), E>;
    fn delete<E>(&self) -> Result<(), E>;
    fn new(item: Event) -> Self;
    async fn set(&self) -> Result<String, String>;
    async fn update<E>(&self) -> Result<(), E>;
    async fn get(key: &str) -> Result<Option<EventModel>, String>;
    fn select<T:Message,E,S>(filter: HashMap<&str, &str, S>) -> Result<Option<Vec<T>>, E>;

}
