

use std::str::FromStr;
use tonic::{transport::Server, Request, Response, Status};
use crate::api::event_service_server::{EventService, EventServiceServer};
use crate::api::{Event, GetEventsRequest, GetEventsRespond, SetEventRequest, SetEventRespond};
use clap::Parser;

#[tonic::async_trait]
impl EventService for Event {
    async fn set(&self, request: Request<SetEventRequest>) -> Result<Response<SetEventRespond>, Status> {
        //println!("Got a request: {:?}", request);

        let reply = SetEventRespond {
            status: String::from_str("Ok").unwrap(),
            id: String::from_str("uuid").unwrap()
        };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetEventsRequest>) -> Result<Response<GetEventsRespond>, Status> {
        println!("Got a request: {:?}", request);
        let mok_event = Event{
            name: String::from_str("Ok-mok").unwrap(),
            partner: 5,
            timestamp: 8888888
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
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let echo = Event::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EventServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}