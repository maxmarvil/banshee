pub mod event;
use std::collections::HashMap;
use sqlx::Type;
use crate::api::Event;
use prost::Message;


pub  trait  Model {
    fn delete_by_id<T, E>(id: T) -> Result<(), E>;
    fn delete<E>(&self) -> Result<(), E>;
    fn new(item: Event) -> Self;
    async fn set(&self) -> Result<(), String>;
    async fn update<E>(&self) -> Result<(), E>;
    fn get<T:Message, E>(key: &str) -> Result<Option<T>, E>;
    fn select<T:Message,E,S>(filter: HashMap<&str, &str, S>) -> Result<Option<Vec<T>>, E>;

}
