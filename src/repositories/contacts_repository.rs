use async_trait::async_trait;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::errors::Error;

/// Default page number.
pub const DEFAULT_PAGE_NO: u32 = 1;

/// Default page size.
pub const DEFAULT_PAGE_SIZE: u32 = 5;

/// Contract for a Contacts repository
#[async_trait]
pub trait ContactsRepository {
    /// Returns all contacts, considering a page_no and page_size.
    /// If no page_no or no page_size, defaults will be used.
    async fn get_all(
        &self,
        page_no: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Contact>, Error>;

    /// Return a single contact, if found, otherwise None.
    async fn get(&self, id: ContactId) -> Result<Option<Contact>, Error>;

    /// Adds a contact to the repository. Returns the new contact.
    async fn add(&mut self, new_contact: NewContact) -> Result<Contact, Error>;

    /// Updates an existing contact. Doesn't return anything. Safe for no-ops.
    async fn update(&mut self, contact: Contact, id: ContactId) -> Result<(), Error>;

    /// Updates only the email of a contact. Doesn't return anything. Safe for no-ops.
    async fn update_email(&mut self, new_email: String, id: ContactId) -> Result<(), Error>;

    /// Updates only the phone_no of a contact. Doesn't return anything. Safe for no-ops.
    async fn update_phone_no(&mut self, new_phone_no: i64, id: ContactId) -> Result<(), Error>;

    /// Deletes a contact. Doesn't return anything. Safe for no-ops.
    async fn delete(&mut self, id: ContactId) -> Result<(), Error>;
}

pub fn get_limit_and_offset(page_no: Option<u32>, page_size: Option<u32>) -> (u32, u32) {
    let page_no: u32 = page_no.unwrap_or(DEFAULT_PAGE_NO);
    let page_size: u32 = page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    (page_size, (page_no - 1) * page_size)
}
