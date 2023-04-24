use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

mod api;
mod middleware;
mod models;
mod repositories;

use crate::api::contacts_routes::get_all_routes;
use crate::repositories::contacts_db_repository::ContactsDbRepository;

const API_PORT_KEY: &str = "API_PORT";
const DEFAULT_API_PORT: &str = "8090";

#[tokio::main]
async fn main() {
    dotenv().expect("Missing .env file");
    env_logger::init();

    let addr: SocketAddr = get_addr();
    let db_repository: ContactsDbRepository = ContactsDbRepository::new().await;
    let routes = get_all_routes(db_repository);

    warp::serve(routes).run(addr).await;
}

fn get_addr() -> SocketAddr {
    let port: String = env::var(API_PORT_KEY).unwrap_or(DEFAULT_API_PORT.to_string());
    let addr_as_str: String = format!("127.0.0.1:{port}");
    addr_as_str
        .parse()
        .unwrap_or_else(|_| panic!("Cannot parse socket address {}", addr_as_str))
}
