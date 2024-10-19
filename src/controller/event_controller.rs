use tonic::{Request, Status};

use sqlx::Error::Database;
use crate::api::{Event, GetEventRequest, GetEventRespond, SetEventRequest, SetEventRespond};
use crate::model::event::EventModel;
use crate::model::{Model};

pub async fn set_new (request: Request<SetEventRequest>) -> Result<SetEventRespond, Status> {
    let data = request.get_ref();

    let event_model = EventModel::new(Event{
        comment: data.comment.clone(),
        partner: data.partner.clone(),
        timestamp: data.timestamp.clone(),
        payload: data.payload.clone(),
    });
    //println!("model new {:#?}", event_model);

    let res = event_model.set().await;

    Ok(SetEventRespond {
        status: format!("Ok"),
        id: res.unwrap()
    })
}

pub async fn get_one(request: Request<GetEventRequest>) -> Result<GetEventRespond, Status> {
    let data = request.get_ref();
    let res = EventModel::get(&data.key).await.unwrap();
    let (status, data) = match res {
        Some(data) =>  ("Ok".to_string(), Some(data.event)) ,
        None => ("Not Found".to_string(), None)
    };

    Ok(GetEventRespond {
        status: status,
        event: data
    })
}