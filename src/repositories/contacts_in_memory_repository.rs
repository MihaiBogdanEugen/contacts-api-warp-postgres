use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::errors::Error;
use crate::repositories::contacts_repository::get_limit_and_offset;

use super::contacts_repository::ContactsRepository;

pub struct ContactsInMemoryRepository {
    data: Arc<RwLock<HashMap<ContactId, Contact>>>,
}

impl ContactsInMemoryRepository {
    pub fn new() -> Self {
        ContactsInMemoryRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for ContactsInMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ContactsRepository for ContactsInMemoryRepository {
    async fn get_all(
        &self,
        page_no: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Contact>, Error> {
        let (limit, offset): (u32, u32) = get_limit_and_offset(page_no, page_size);
        let contacts: Vec<Contact> = self
            .data
            .read()
            .await
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        Ok(contacts)
    }

    async fn get(&self, id: ContactId) -> Result<Option<Contact>, Error> {
        match self.data.read().await.get(&id) {
            Some(contact) => Ok(Some(contact.to_owned())),
            None => Ok(None),
        }
    }

    async fn add(&mut self, new_contact: NewContact) -> Result<Contact, Error> {
        let mut id: i32 = self
            .data
            .read()
            .await
            .values()
            .count()
            .try_into()
            .map_err(Error::NumTryFromIntError)?;
        while self.data.read().await.contains_key(&ContactId(id)) {
            id += 1;
        }

        let contact: Contact = Contact {
            id: ContactId(id),
            name: new_contact.name,
            phone_no: new_contact.phone_no,
            email: new_contact.email,
        };
        self.data
            .write()
            .await
            .insert(ContactId(id), contact.clone());

        Ok(contact)
    }

    async fn update(&mut self, contact: Contact, id: ContactId) -> Result<(), Error> {
        self.data.write().await.insert(id, contact.clone());
        Ok(())
    }

    async fn update_email(&mut self, new_email: String, id: ContactId) -> Result<(), Error> {
        if let Some(contact) = self.data.read().await.get(&id) {
            self.data.write().await.insert(
                id,
                Contact {
                    email: new_email,
                    ..contact.clone()
                },
            );
        }
        Ok(())
    }

    async fn update_phone_no(&mut self, new_phone_no: i64, id: ContactId) -> Result<(), Error> {
        if let Some(contact) = self.data.read().await.get(&id) {
            self.data.write().await.insert(
                id,
                Contact {
                    phone_no: new_phone_no,
                    ..contact.clone()
                },
            );
        }
        Ok(())
    }

    async fn delete(&mut self, id: ContactId) -> Result<(), Error> {
        self.data.write().await.remove_entry(&id);
        Ok(())
    }
}
