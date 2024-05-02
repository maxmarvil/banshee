pub mod event;
use std::collections::HashMap;
pub trait Message {}

pub trait Model {
    fn delete<E>(&self) -> Result<(), E>;
}

pub trait DBModel:Model {
    fn set<T:Message, E>(data: T) -> Result<(), E>;
    fn get<T:Message, E>(key: &str) -> Result<Option<T>, E>;
    fn select<T:Message,E,S>(filter: HashMap<&str, &str, S>) -> Result<Option<Vec<T>>, E>;
    fn delete<T, E>(id: T) -> Result<(), E>;
}