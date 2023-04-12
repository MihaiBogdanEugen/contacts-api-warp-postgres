use async_trait::async_trait;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;

#[async_trait]
pub trait ContactsRepository {
    async fn get_all(
        &self,
        page_no: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Contact>, String>;

    async fn get(&self, id: ContactId) -> Result<Option<Contact>, String>;

    async fn add(&self, new_contact: NewContact) -> Result<Contact, String>;

    async fn update(&self, contact: Contact, id: ContactId) -> Result<Contact, String>;

    async fn delete(&self, id: ContactId) -> Result<bool, String>;
}
