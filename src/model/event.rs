use std::collections::HashMap;
use redis::{Connection, ErrorKind, from_redis_value, FromRedisValue, NumericBehavior, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::__private::de::IdentifierDeserializer;
use serde::ser::SerializeStruct;
use crate::api::{Event};
use crate::model::{DBModel, Message, Model};

pub struct EventModel {
    pub event: Event,
    pub con: Connection
}

impl Message for Event {
    // fn update_struct(&mut self, data: Event) -> () {
    //     self.comment = data.comment.clone();
    //     self.partner = data.partner.clone();
    //     self.timeout = data.timeout.clone();
    //     self.payload = data.payload.clone();
    //     ()
    // }
}
impl  FromRedisValue for Event {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let json_str: String = from_redis_value(v)?;
        println!("dump {:#?}",json_str);
        // let result: Self = match serde_json::from_str::<Self>(&json_str) {
        //     Ok(v) => v,
        //     Err(_err) => return Err((ErrorKind::TypeError, "Parse to JSON Failed").into())
        // };

        Ok(Event{
            comment:"mok".to_string(),
            partner:4,
            timeout:2324,
            payload: "".to_string()
        })
    }

    fn from_owned_redis_value(v: Value) -> RedisResult<Self> {
        Self::from_redis_value(&v)
    }

    fn from_redis_values(items: &[Value]) -> RedisResult<Vec<Self>> {
        items.iter().map(FromRedisValue::from_redis_value).collect()
    }

    fn from_owned_redis_values(items: Vec<Value>) -> RedisResult<Vec<Self>> {
        items
            .into_iter()
            .map(FromRedisValue::from_owned_redis_value)
            .collect()
    }

    fn from_byte_vec(_vec: &[u8]) -> Option<Vec<Self>> {
        Self::from_owned_redis_value(Value::Data(_vec.into()))
            .map(|rv| vec![rv])
            .ok()
    }

    fn from_owned_byte_vec(_vec: Vec<u8>) -> RedisResult<Vec<Self>> {
        Self::from_owned_redis_value(Value::Data(_vec)).map(|rv| vec![rv])
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Event", 4)?;
        s.serialize_field("comment", &self.comment)?;
        s.serialize_field("partner", &self.partner)?;
        s.serialize_field("timeout", &self.timeout)?;
        s.serialize_field("payload", &self.payload)?;
        s.end()
    }
}

// impl Deserialize for Event {
//     fn deserialize< D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
//         let fields: &[&str] = &["comment", "partner","timeout","payload"];
//         deserializer.deserialize_struct("Event", fields, None)?
//     }
//
//     fn deserialize_in_place< D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer {
//         *place = match Deserialize::deserialize(deserializer) {
//             Ok(val) => val,
//             Err(err) => return Err(err),
//         };
//         Ok(())
//     }
// }

impl Model for EventModel {
    fn delete<E>(&self)->Result<(), E> {
        Ok(())
    }
}

impl DBModel for EventModel {
    fn set<T:Message, E>(data: T, connection: Connection) -> Result<(), E> {
        //todo!()
        Ok(())
    }

    fn get<T:Message, E>(key: &str, connection: Connection) -> Result<Option<T>, E> {
        //todo!()
        Ok(None)
    }

    fn select<T:Message, E, S>(filter: HashMap<&str, &str, S>, connection: Connection) -> Result<Option<Vec<T>>, E> {
        //todo!()
        Ok(None)
    }

    fn delete<T, E>(key: T, connection: Connection) -> Result<(), E> {
        //todo!()
        Ok(())
    }
}