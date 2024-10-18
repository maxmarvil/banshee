use std::collections::HashMap;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::error;
use redis::{Connection, ErrorKind, from_redis_value, FromRedisValue, NumericBehavior, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::__private::de::IdentifierDeserializer;
use serde::ser::SerializeStruct;
use sqlx::{Error, Execute};
use sqlx::error::DatabaseError;
use uuid::Uuid;
use prost::Message;
use crate::api::{Event};
use crate::connection;
use crate::model::{ Model};
use chrono::prelude::*;

#[derive(Debug)]
pub struct EventModel {
    pub event: Event,
}

impl FromRedisValue for Event {
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
            timestamp:1718285363,
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
        s.serialize_field("timestamp", &self.timestamp)?;
        s.serialize_field("payload", &self.payload)?;
        s.end()
    }
}

// impl Deserialize for Event {
//     fn deserialize< D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
//         let fields: &[&str] = &["comment", "partner","timestamp","payload"];
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
    fn delete_by_id<T, E>(key: T) -> Result<(), E> {
        //todo!()
        Ok(())
    }

    fn delete<E>(&self) -> Result<(), E> {
        //todo!()
        Ok(())
    }

    fn new(item: Event) -> EventModel{
        EventModel {
            event: item,
        }
    }

    async fn set(&self) -> Result<String, String> {
        let  key = Uuid::new_v4();
        let mut pool_result = connection::mysql_connection::connect().await;
        let conn_pool = match pool_result {
            Ok(pool) => pool,
            Err(e) => panic!("Ошибка соединения: {:#?}", e)
        };

        // Create a normal DateTime from the NaiveDateTime
        let datetime  = DateTime::from_timestamp(self.event.timestamp, 0).unwrap();
        let q_builder = sqlx::query!(
            r#"INSERT INTO events (id, partner_id, timestamp, comment, payload) VALUES (?, ?, ?, ?, ?);"#,
            key.to_string(),
            self.event.partner,
            datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            self.event.comment.clone(),
            self.event.payload.clone()
        );
        let res =  q_builder.execute(&conn_pool).await;

        let row = match res {
            Ok(new) => new ,
            Err(er) => panic!("Ошибка запроса: {:#?}", er)
        };

        Ok(key.to_string())
    }
    async fn update<E>(&self) -> Result<(), E> {
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
}
