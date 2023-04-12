use dotenv::dotenv;
use sqlx::Connection;
use sqlx::PgConnection;
use std::env;

use crate::models::contact::Contact;
use crate::models::contact::NewContact;
use crate::repository::contacts_db_repository::ContactsDbRepository;
use crate::repository::contacts_repository::ContactsRepository;

mod models;
mod repository;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

#[tokio::main]
async fn main() {
    dotenv().expect("Missing .env file");

    let db_url: String = env::var(DATABASE_URL_KEY)
        .unwrap_or_else(|_| panic!("Missing environment variable {DATABASE_URL_KEY}"));

    let mut db_connection: PgConnection = PgConnection::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to db {}", db_url));

    sqlx::migrate!()
        .run(&mut db_connection)
        .await
        .expect("cannot run migrations");

    let new_contact: NewContact = NewContact {
        name: "Sorin Mihai".to_string(),
        phone_no: 492345678901,
        email: "sorin.mihai@mail.com".to_string(),
    };

    let db_repository: ContactsDbRepository = ContactsDbRepository::new(&db_url).await;
    let add_result: Result<Contact, String> = db_repository.add(new_contact).await;

    match add_result {
        Ok(x) => println!("{:?}", x),
        Err(x) => println!("{}", x),
    }

    println!("Hello, world!");
}
