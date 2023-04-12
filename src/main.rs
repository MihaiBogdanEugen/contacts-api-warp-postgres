use dotenv::dotenv;
use std::env;

use crate::repository::contacts_db_repository::ContactsDbRepository;
use crate::models::contact::NewContact; 
use crate::models::contact::Contact; 
use crate::repository::contacts_repository::ContactsRepository;

mod models;
mod repository;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

#[tokio::main]
async fn main() {

    dotenv().expect("Missing .env file");
    let db_url: String = env::var(DATABASE_URL_KEY).unwrap_or_else(|_| panic!("Missing environment variable {DATABASE_URL_KEY}"));

    let db_repository: ContactsDbRepository = ContactsDbRepository::new(&db_url).await;

    let new_contact: NewContact = NewContact {
        name: "Bogdan Mihai".to_string(),
        phone_no: 491234567890,
        email: "bogdan.mihai@mail.com".to_string()
    };
    let add_result: Result<Contact, String> = db_repository.add(new_contact).await;
    match add_result {
        Ok(x) => println!("{:?}", x),
        Err(x) => println!("{}", x),
    }

    println!("Hello, world!");
}
