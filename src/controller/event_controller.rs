use tonic::{Request, Status};
use std::str::FromStr;
use redis::{ RedisError, RedisResult};
use uuid::Uuid;

use sqlx::Error::Database;
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

    let  key = Uuid::new_v4();
    //conection


    Ok(SetEventRespond {
        status: format!("Ok"),
        id: String::from_str(&key.to_string()).unwrap()
    })
}

pub async fn get_one(request: Request<GetEventRequest>) -> Result<GetEventRespond, Status> {
    Ok(
        GetEventRespond {
            status: format!("Ok-mok"),
            event: None//Some(event_val)
        })
}
// pub async fn get_one(request: Request<GetEventRequest>) -> Result<GetEventRespond, RedisError> {
//     let data = request.get_ref();
//     let mut conection = connection::redis_connection::connect().await.unwrap();
//     let item:RedisResult<Event> = redis::pipe()
//         .atomic()
//         .get(data.key.clone())
//         .query_async(&mut conection)
//         .await;
//
//     let item = match item {
//         RedisResult::Err(err) => return Err(err),
//         RedisResult::Ok(val) => val
//     };
//     //println!("dump {:#?}", item);
//
//
//     // let (redis_res) = match item {
//     //     Ok(val)=>val,
//     //     Err(er) => {
//     //         return Err(Status::new(Code::NotFound, "Not found record"))
//     //     }
//     // };
//     println!("redis value {:#?}", item);
//     //let event_val = Event::try_from(redis_res).unwrap();
//     Ok(
//         GetEventRespond {
//             status: format!("Ok"),
//             event: None//Some(event_val)
//         }
//     )
// }