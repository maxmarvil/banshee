use tonic::{Code, Request, Response, Status};
use std::str::FromStr;
use prost::bytes::Bytes;
use redis::{FromRedisValue, RedisError, RedisResult};
use tonic::metadata::MetadataMap;
use crate::{connection, calculate_hash};
use crate::api::{Event, GetEventRequest, GetEventRespond, GetEventsRequest, GetEventsRespond, SetEventRequest, SetEventRespond};

pub async fn set_new (request: Request<SetEventRequest>) -> Result<SetEventRespond, Status> {
    let data = request.get_ref();
    let new_event = Event{
        comment: data.comment.clone(),
        partner: data.partner.clone(),
        timeout: data.timeout.clone(),
        payload: data.comment.clone(),
    };

    let mut conection = connection::redis_connection::connect().await.unwrap();
    let  key = calculate_hash(&new_event);

    let _: () = redis::pipe()
        .atomic()
        .set(format!("{}:partner-{}:{key}",new_event.timeout,new_event.partner), serde_json::to_string(&new_event.clone()).unwrap())
        .query_async(&mut conection)
        .await.unwrap();
    println!("Got a request: {key} {:#?}", &new_event);

    Ok(SetEventRespond {
        status: format!("Ok"),
        id: String::from_str("uuid").unwrap()
    })
}
pub async fn get_one(request: Request<GetEventRequest>) -> Result<GetEventRespond, RedisError> {
    let data = request.get_ref();
    let mut conection = connection::redis_connection::connect().await.unwrap();
    let item:RedisResult<Event> = redis::pipe()
        .atomic()
        .get(data.key.clone())
        .query_async(&mut conection)
        .await;

    let item = match item {
        RedisResult::Err(err) => return Err(err),
        RedisResult::Ok(val) => val
    };
    //println!("dump {:#?}", item);


    // let (redis_res) = match item {
    //     Ok(val)=>val,
    //     Err(er) => {
    //         return Err(Status::new(Code::NotFound, "Not found record"))
    //     }
    // };
    println!("redis value {:#?}", item);
    //let event_val = Event::try_from(redis_res).unwrap();
    Ok(
        GetEventRespond {
            status: format!("Ok"),
            event: None//Some(event_val)
        }
    )
}