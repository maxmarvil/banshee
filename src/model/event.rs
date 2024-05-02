use std::collections::HashMap;
use redis::{Connection, NumericBehavior, RedisWrite, ToRedisArgs, };
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::api::{Event};
use crate::model::{DBModel, Message, Model};

#[derive(Serialize, Deserialize)]
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

impl Serialize for Event{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Event", 4)?;
        s.serialize_field("comment", &self.comment)?;
        s.serialize_field("partner", &self.partner)?;
        s.serialize_field("timeout", &self.timeout)?;
        s.serialize_field("payload", &self.payload)?;
        s.end()
    }
}



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