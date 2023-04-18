use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contact {
    pub id: ContactId,
    pub name: String,
    pub phone_no: i64,
    pub email: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContactId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewContact {
    pub name: String,
    pub phone_no: i64,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateContactEmail {
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateContactPhoneNo {
    pub phone_no: i64,
}
