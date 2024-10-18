use std::str::FromStr;
use clap::Parser;
use tonic::{Request, Response, Status, transport::Server};
use model::event::EventModel;
use api::{Event, GetEventsRequest, GetEventsRespond, SetEventRequest, SetEventRespond,GetEventRequest, GetEventRespond};
use api::event_service_server::{EventService, EventServiceServer};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc};
use log::info;
use serde::{Serialize,ser::{SerializeStruct}};
use serde_json::{Serializer};
use sqlx::{MySql, Pool};
use tokio::sync::Mutex;

pub mod api;
pub mod model;
pub mod connection;
pub mod controller;
mod validator;

//const BD_CONNECT: Arc<Mutex<Option<&Pool<MySql>>>> = Arc::new(Mutex::new(None));
#[tonic::async_trait]
impl EventService for Event {
    async fn set(&self, request: Request<SetEventRequest>) -> Result<Response<SetEventRespond>, Status> {
        let  result = controller::event_controller::set_new(request);

        Ok(Response::new(result.await.unwrap()))
    }

    async fn get(&self, request: Request<GetEventRequest>) -> Result<Response<GetEventRespond>, Status> {
        println!("Got a request: {:#?}", request);

        let  result = controller::event_controller::get_one(request);

        // let reply = GetEventRespond {
        //     status:  String::from_str("Ok").unwrap(),
        //     event: Some(mok_event)
        // };

        Ok(Response::new(result.await.unwrap()))
    }

    async fn get_list(&self, request: Request<GetEventsRequest>) -> Result<Response<GetEventsRespond>, Status> {
        let mok_event = Event{
            comment: String::from_str("Ok-mok").unwrap(),
            partner: 5,
            timestamp: 1718285363,
            payload: String::from_str("{}").unwrap()
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

    let pool_result = connection::mysql_connection::connect().await;
    //let mut opt_pool = connect_db;

    let pool  = match pool_result {
        Ok(pool) => Some(pool),
        Err(_) => None
    };


    println!("Server listening on {}", addr);
    info!("try migration");

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