use async_trait::async_trait;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::errors::Error;

/// Default page number.
pub(crate) const DEFAULT_PAGE_NO: u32 = 1;

/// Default page size.
pub(crate) const DEFAILT_PAGE_SIZE: u32 = 5;

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

    /// Adds a contact to the repository.
    async fn add(&self, new_contact: NewContact) -> Result<Contact, Error>;

    /// Updates an existing contact.
    async fn update(&self, contact: Contact, id: ContactId) -> Result<Contact, Error>;

    /// Deletes a contact.
    async fn delete(&self, id: ContactId) -> Result<bool, Error>;
}
