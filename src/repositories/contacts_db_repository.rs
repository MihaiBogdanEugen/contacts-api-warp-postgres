use std::env;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use sqlx::Connection;
use sqlx::PgConnection;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::errors::Error;

use super::contacts_repository::get_limit_and_offset;
use super::contacts_repository::ContactsRepository;

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const MAX_CONNECTIONS: u32 = 5;

const SQL_SELECT_PAGE: &str = "SELECT id, name, phone_no, email FROM contacts LIMIT $1 OFFSET $2;";
const SQL_SELECT_ONE: &str = "SELECT id, name, phone_no, email FROM contacts WHERE id = $1;";
const SQL_INSERT: &str = "INSERT INTO contacts(name, phone_no, email) VALUES ($1, $2, $3) RETURNING id, name, phone_no, email;";
const SQL_UPDATE: &str = "UPDATE contacts SET name = $1, phone_no = $2, email = $3 WHERE id = $4;";
const SQL_UPDATE_EMAIL: &str = "UPDATE contacts SET email = $1 WHERE id = $2;";
const SQL_UPDATE_PHONE_NO: &str = "UPDATE contacts SET phone_no = $1 WHERE id = $2;";
const SQL_DELETE: &str = "DELETE FROM contacts WHERE id = $1;";

#[derive(Debug, Clone)]
pub struct ContactsDbRepository {
    db_pool: Pool<Postgres>,
}

impl ContactsDbRepository {
    pub async fn new() -> Self {
        let db_url: String = env::var(DATABASE_URL_KEY)
            .unwrap_or_else(|_| panic!("Missing environment variable: {DATABASE_URL_KEY}"));

        Self::run_migrations(&db_url).await;

        PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&db_url)
            .await
            .map(|db_pool: Pool<Postgres>| ContactsDbRepository { db_pool })
            .unwrap_or_else(|_| panic!("Couldn't establish a DB connection: {db_url}"))
    }

    async fn run_migrations(db_url: &String) {
        let mut db_connection: PgConnection = PgConnection::connect(db_url)
            .await
            .unwrap_or_else(|_| panic!("Couldn't establish a DB connection: {db_url}"));

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
        sqlx::query(SQL_SELECT_PAGE)
            .bind(limit as i32)
            .bind(offset as i32)
            .map(map_row)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|err: sqlx::Error| Error::Db(err.to_string()))
    }

    async fn get(&self, id: ContactId) -> Result<Option<Contact>, Error> {
        sqlx::query(SQL_SELECT_ONE)
            .bind(id.0)
            .map(map_row)
            .fetch_one(&self.db_pool)
            .await
            .map(Some)
            .or_else(|err: sqlx::Error| match err {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(Error::Db(err.to_string())),
            })
    }

    async fn add(&mut self, new_contact: NewContact) -> Result<Contact, Error> {
        sqlx::query(SQL_INSERT)
            .bind(new_contact.name)
            .bind(new_contact.phone_no)
            .bind(new_contact.email)
            .map(map_row)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|err: sqlx::Error| Error::Db(err.to_string()))
    }

    async fn update(&mut self, contact: Contact, id: ContactId) -> Result<(), Error> {
        sqlx::query(SQL_UPDATE)
            .bind(contact.name)
            .bind(contact.phone_no)
            .bind(contact.email)
            .bind(id.0)
            .execute(&self.db_pool)
            .await
            .map(|_| ())
            .map_err(|err: sqlx::Error| match err {
                sqlx::Error::RowNotFound => Error::NotFound { id: id.0 },
                _ => Error::Db(err.to_string()),
            })
    }

    async fn update_email(&mut self, new_email: String, id: ContactId) -> Result<(), Error> {
        sqlx::query(SQL_UPDATE_EMAIL)
            .bind(new_email)
            .bind(id.0)
            .execute(&self.db_pool)
            .await
            .map(|_| ())
            .map_err(|err: sqlx::Error| match err {
                sqlx::Error::RowNotFound => Error::NotFound { id: id.0 },
                _ => Error::Db(err.to_string()),
            })
    }

    async fn update_phone_no(&mut self, new_phone_no: i64, id: ContactId) -> Result<(), Error> {
        sqlx::query(SQL_UPDATE_PHONE_NO)
            .bind(new_phone_no)
            .bind(id.0)
            .execute(&self.db_pool)
            .await
            .map(|_| ())
            .map_err(|err: sqlx::Error| match err {
                sqlx::Error::RowNotFound => Error::NotFound { id: id.0 },
                _ => Error::Db(err.to_string()),
            })
    }

    async fn delete(&mut self, id: ContactId) -> Result<(), Error> {
        sqlx::query(SQL_DELETE)
            .bind(id.0)
            .execute(&self.db_pool)
            .await
            .map(|_| ())
            .map_err(|err: sqlx::Error| Error::Db(err.to_string()))
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
