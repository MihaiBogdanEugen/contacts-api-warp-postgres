use std::collections::HashMap;

use warp::hyper::StatusCode;
use warp::reject::Reject;
use warp::Rejection;
use warp::Reply;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::contact::UpdateContactEmail;
use crate::models::contact::UpdateContactPhoneNo;
use crate::models::errors::Error;
use crate::repositories::contacts_repository::ContactsRepository;

const PAGE_NO_KEY: &str = "page_no";
const PAGE_SIZE: &str = "page_size";

pub async fn get_all_contacts(
    query_parameters: HashMap<String, String>,
    contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    let pagination: Pagination = get_pagination(query_parameters)?;
    contacts_repository
        .get_all(pagination.page_no, pagination.page_size)
        .await
        .map(|contacts: Vec<Contact>| warp::reply::json(&contacts))
        .map_err(warp::reject::custom)
}

pub async fn get_contact(
    id: i32,
    contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    let possible_contact: Option<Contact> = match contacts_repository.get(ContactId(id)).await {
        Ok(x) => x,
        Err(err) => return Err(warp::reject::custom(err)),
    };
    possible_contact
        .ok_or(warp::reject::custom(Error::NotFound { id }))
        .map(|contact: Contact| warp::reply::json(&contact))
}

pub async fn add_conact(
    new_contact: NewContact,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    contacts_repository
        .add(new_contact)
        .await
        .map(|contact: Contact| warp::reply::json(&contact))
        .map_err(warp::reject::custom)
}

pub async fn update_contact(
    id: i32,
    contact: Contact,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    contacts_repository
        .update(contact, ContactId(id))
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(warp::reject::custom)
}

pub async fn update_contact_email(
    id: i32,
    payload: UpdateContactEmail,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    contacts_repository
        .update_email(payload.email, ContactId(id))
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(warp::reject::custom)
}

pub async fn update_contact_phone_no(
    id: i32,
    payload: UpdateContactPhoneNo,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    contacts_repository
        .update_phone_no(payload.phone_no, ContactId(id))
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(warp::reject::custom)
}

pub async fn delete_contact(
    id: i32,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    contacts_repository
        .delete(ContactId(id))
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(warp::reject::custom)
}

struct Pagination {
    page_no: Option<u32>,
    page_size: Option<u32>,
}

fn get_pagination(query_parameters: HashMap<String, String>) -> Result<Pagination, Error> {
    let mut page_no: Option<u32> = None;
    if query_parameters.contains_key(PAGE_NO_KEY) {
        page_no = match query_parameters.get(PAGE_NO_KEY).unwrap().parse::<u32>() {
            Ok(val) => Some(val),
            Err(error) => return Err(Error::StringToU32(error.to_string())),
        };
    }

    let mut page_size: Option<u32> = None;
    if query_parameters.contains_key(PAGE_SIZE) {
        page_size = match query_parameters.get(PAGE_SIZE).unwrap().parse::<u32>() {
            Ok(val) => Some(val),
            Err(error) => return Err(Error::StringToU32(error.to_string())),
        };
    }

    Ok(Pagination { page_no, page_size })
}

pub async fn handle_rejection(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::StringToU32(message)) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            message.to_owned(),
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(Error::Db(message)) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            message.to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::NotFound { id }) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            id.to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Bad request of route not found".to_string(),
            StatusCode::BAD_REQUEST,
        ))
    }
}

impl Reject for Error {}
