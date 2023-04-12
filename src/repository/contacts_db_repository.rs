use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use sqlx::Row;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;

use super::contacts_repository::ContactsRepository;

const MAX_CONNECTIONS: u32 = 5;
const DEFAULT_PAGE_NO: u32 = 1;
const DEFAILT_PAGE_SIZE: u32 = 5;

#[derive(Debug, Clone)]
pub struct ContactsDbRepository {
    pub db_pool: Pool<Postgres>,
}

impl ContactsDbRepository {
    pub async fn new(db_url: &str) -> Self {
        match PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(db_url)
            .await {
                Ok(db_pool) => ContactsDbRepository { db_pool },
                Err(e) => panic!("Couldn't establish DB connection: {}", e),
            }
    }
}

#[async_trait]
impl ContactsRepository for ContactsDbRepository {
    
    async fn get_all(
        &self,
        page_no: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Contact>, String> {
        let page_no: u32 = page_no.unwrap_or(DEFAULT_PAGE_NO);
        let page_size: u32 = page_size.unwrap_or(DEFAILT_PAGE_SIZE);

        let limit: u32 = page_size;
        let offset: u32 = (page_no - 1) * page_size;

        match sqlx::query("SELECT id, name, phone_no, email FROM contacts LIMIT $1 OFFSET $2;")
            .bind(limit as i32)
            .bind(offset as i32)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                phone_no: row.get("phone_no"),
                email: row.get("email"),
            })
            .fetch_all(&self.db_pool)
            .await {
                Ok(contacts) => Ok(contacts),
                Err(db_error) => Err(db_error.to_string()),
            }
    }

    async fn get(&self, id: ContactId) -> Result<Option<Contact>, String> {
        match sqlx::query("SELECT id, name, phone_no, email FROM contacts WHERE id = $1;")
            .bind(id.0)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                phone_no: row.get("phone_no"),
                email: row.get("email"),
            })
            .fetch_one(&self.db_pool)
            .await {
                Ok(contact) => Ok(Some(contact)),
                Err(db_error) => Err(db_error.to_string()),
            }
    }

    async fn add(&self, new_contact: NewContact) -> Result<Contact, String> {
        match sqlx::query("INSERT INTO contacts(name, phone_no, email) VALUES ($1, $2, $3) RETURNING id, name, phone_no, email;")
            .bind(new_contact.name)
            .bind(new_contact.phone_no)
            .bind(new_contact.email)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                phone_no: row.get("phone_no"),
                email: row.get("email")
            })
            .fetch_one(&self.db_pool)
            .await {
                Ok(contact) => Ok(contact),
                Err(db_error) => Err(db_error.to_string()),
            }
    }

    async fn update(&self, contact: Contact, id: ContactId) -> Result<Contact, String> {
        match sqlx::query("UPDATE contacts SET name = $1, phone_no = $2, email = $3 WHERE id = $4 RETURNING id, name, phone_no, email;")
            .bind(contact.name)
            .bind(contact.phone_no)
            .bind(contact.email)
            .bind(id.0)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                phone_no: row.get("phone_no"),
                email: row.get("email")
            })
            .fetch_one(&self.db_pool)
            .await {
                Ok(contact) => Ok(contact),
                Err(db_error) => Err(db_error.to_string()),
            }
    }

    async fn delete(&self, id: ContactId) -> Result<bool, String> {
        match sqlx::query("DELETE FROM contacts WHERE id = $1;")
            .bind(id.0)
            .execute(&self.db_pool)
            .await {
                Ok(_) => Ok(true),
                Err(db_error) => Err(db_error.to_string()),
            }
    }
}
