use std::collections::HashMap;
use redis::Connection;
use crate::api::{Event};
use crate::model::{DBModel, Message, Model};

pub struct EventModel<'a>{
    id: &'a str,
    event: Event,
    con: Connection
}
impl Message for Event{}
impl Model for EventModel<'_> {
    fn delete<E>(&self)->Result<(), E> {
        Ok(())
    }
}

impl DBModel for EventModel<'_> {
    fn set<T:Message, E>(data: T) -> Result<(), E> {
        //todo!()
        Ok(())
    }

    fn get<T:Message, E>(key: &str) -> Result<Option<T>, E> {
        //todo!()
        Ok(None)
    }

    fn select<T:Message, E, S>(filter: HashMap<&str, &str, S>) -> Result<Option<Vec<T>>, E> {
        //todo!()
        Ok(None)
    }

    fn delete<T, E>(key: T) -> Result<(), E> {
        //todo!()
        Ok(())
    }
}