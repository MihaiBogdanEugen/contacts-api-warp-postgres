use std::env;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use sqlx::Connection;
use sqlx::PgConnection;
use sqlx::PgPool;
use sqlx::Row;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::errors::Error;

use super::contacts_repository::get_limit_and_offset;
use super::contacts_repository::ContactsRepository;

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const MAX_CONNECTIONS: u32 = 5;

#[derive(Debug, Clone)]
pub struct ContactsDbRepository {
    db_pool: PgPool,
}

impl ContactsDbRepository {
    pub async fn new() -> Self {
        let db_url: String = env::var(DATABASE_URL_KEY)
            .unwrap_or_else(|_| panic!("Missing environment variable {DATABASE_URL_KEY}"));

        Self::run_migrations(&db_url).await;

        match PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&db_url)
            .await
        {
            Ok(db_pool) => ContactsDbRepository { db_pool },
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        }
    }

    async fn run_migrations(db_url: &String) {
        let mut db_connection: PgConnection = PgConnection::connect(db_url)
            .await
            .unwrap_or_else(|_| panic!("Cannot connect to db {}", db_url));

        sqlx::migrate!()
            .run(&mut db_connection)
            .await
            .unwrap_or_else(|_| panic!("Cannot run migrations"));
    }
}

#[async_trait]
impl ContactsRepository for ContactsDbRepository {
    async fn get_all(
        &self,
        page_no: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Contact>, Error> {
        let (limit, offset): (u32, u32) = get_limit_and_offset(page_no, page_size);

        match sqlx::query("SELECT id, name, phone_no, email FROM contacts LIMIT $1 OFFSET $2;")
            .bind(limit as i32)
            .bind(offset as i32)
            .map(map_row)
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(contacts) => Ok(contacts),
            Err(err) => Err(Error::Db(err)),
        }
    }

    async fn get(&self, id: ContactId) -> Result<Option<Contact>, Error> {
        match sqlx::query("SELECT id, name, phone_no, email FROM contacts WHERE id = $1;")
            .bind(id.0)
            .map(map_row)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(contact) => Ok(Some(contact)),
            Err(err) => Err(Error::Db(err)),
        }
    }

    async fn add(&mut self, new_contact: NewContact) -> Result<Contact, Error> {
        match sqlx::query(
            "INSERT INTO contacts(name, phone_no, email) VALUES ($1, $2, $3) RETURNING id, name, phone_no, email;",
        )
        .bind(new_contact.name)
        .bind(new_contact.phone_no)
        .bind(new_contact.email)
        .map(map_row)
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(contact) => Ok(contact),
            Err(err) => Err(Error::Db(err)),
        }
    }

    async fn update(&mut self, contact: Contact, id: ContactId) -> Result<Contact, Error> {
        match sqlx::query("UPDATE contacts SET name = $1, phone_no = $2, email = $3 WHERE id = $4 RETURNING id, name, phone_no, email;")
            .bind(contact.name)
            .bind(contact.phone_no)
            .bind(contact.email)
            .bind(id.0)
            .map(map_row)
            .fetch_one(&self.db_pool)
            .await {
                Ok(contact) => Ok(contact),
                Err(err) => Err(Error::Db(err)),
            }
    }

    async fn delete(&mut self, id: ContactId) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM contacts WHERE id = $1;")
            .bind(id.0)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => Err(Error::Db(err)),
        }
    }
}

fn map_row(row: PgRow) -> Contact {
    Contact {
        id: ContactId(row.get("id")),
        name: row.get("name"),
        phone_no: row.get("phone_no"),
        email: row.get("email"),
    }
}
