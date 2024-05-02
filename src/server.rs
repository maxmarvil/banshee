use std::str::FromStr;
use clap::Parser;
use tonic::{Request, Response, Status, transport::Server};
use model::event::EventModel;
use api::{Event, GetEventsRequest, GetEventsRespond, SetEventRequest, SetEventRespond};
use api::event_service_server::{EventService, EventServiceServer};
use std::hash::{DefaultHasher, Hash, Hasher};
use serde::{Serialize,ser::{SerializeStruct}};
use serde_json::{Serializer};

pub mod api;
pub mod model;
mod controller;

#[tonic::async_trait]
impl EventService for Event {
    async fn set(&self, request: Request<SetEventRequest>) -> Result<Response<SetEventRespond>, Status> {

        let data = request.get_ref();
        let new_event = Event{
            comment: data.comment.clone(),
            partner: data.partner.clone(),
            timeout: data.timeout.clone(),
            payload: data.comment.clone(),
        };

        let mut conection = controller::redis_adapter::connect().await.unwrap();
        let key = calculate_hash(&new_event);
        //let mut event_model = EventModel{event : new_event.clone(), con:conection};

        let _: () = redis::pipe()
            .atomic()
            .set(format!("{}:partner-{}:{key}",new_event.timeout,new_event.partner), serde_json::to_string(&new_event.clone()).unwrap())
            .query_async(&mut conection)
            .await.unwrap();
        println!("Got a request: {key} {:#?}", &new_event);
        let reply = SetEventRespond {
            status: format!("Ok {}", request.get_ref().comment),
            id: String::from_str("uuid").unwrap()
        };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetEventsRequest>) -> Result<Response<GetEventsRespond>, Status> {
        println!("Got a request: {:#?}", request);
        let mok_event = Event{
            comment: String::from_str("Ok-mok").unwrap(),
            partner: 5,
            timeout: 8888888,
            payload: String::from_str("").unwrap()
        };
        let mut resp_vec = vec!();
        resp_vec.push(mok_event);
        let reply = GetEventsRespond {
            status:  String::from_str("Ok").unwrap(),
            events: resp_vec
        };

        Ok(Response::new(reply))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo-server - a simple echo microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let event = Event::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EventServiceServer::new(event))
        .serve(addr)
        .await?;

    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}