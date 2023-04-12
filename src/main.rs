use dotenv::dotenv;
use sqlx::Connection;
use sqlx::PgConnection;
use std::env;
use std::net::SocketAddr;

mod handlers;
mod models;
mod repository;
mod routes;

use crate::repository::contacts_db_repository::ContactsDbRepository;
use crate::routes::contacts_routes::get_all_routes;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

#[tokio::main]
async fn main() {
    dotenv().expect("Missing .env file");
    env_logger::init();

    let port: String = env::var("API_PORT").unwrap_or("8090".to_string());
    let addr_as_str: String = format!("127.0.0.1:{port}");
    let addr: SocketAddr = addr_as_str
        .parse()
        .unwrap_or_else(|_| panic!("Cannot parse socket address {}", addr_as_str));

    let db_url: String = env::var(DATABASE_URL_KEY)
        .unwrap_or_else(|_| panic!("Missing environment variable {DATABASE_URL_KEY}"));

    let mut db_connection: PgConnection = PgConnection::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to db {}", db_url));

    sqlx::migrate!()
        .run(&mut db_connection)
        .await
        .expect("cannot run migrations");

    let db_repository: ContactsDbRepository = ContactsDbRepository::new(&db_url).await;

    let routes = get_all_routes(db_repository);

    warp::serve(routes).run(addr).await;
}
