use std::collections::HashMap;
use redis::{Connection};
use serde::ser::{Serialize, Serializer};
use crate::api::{Event};
use crate::model::{DBModel, Message, Model};

#[derive(Serialize)]
pub struct DataModel {
    comment: String ,
    partner: u32 ,
    timeout: u64,
    payload: String,
}

pub struct EventModel{
    pub event: Event,
    pub con: Connection
}
impl Message for Event{}

impl Model for EventModel {
    fn delete<E>(&self)->Result<(), E> {
        Ok(())
    }
}

impl DBModel for EventModel {
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