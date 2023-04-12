use std::collections::HashMap;

use warp::hyper::StatusCode;
use warp::Rejection;
use warp::Reply;

use crate::models::contact::Contact;
use crate::models::contact::ContactId;
use crate::models::contact::NewContact;
use crate::models::errors::Error;
use crate::repositories::contacts_repository::ContactsRepository;

const PAGE_NO_KEY: &str = "page_no";
const PAGE_SIZE: &str = "page_size";

pub async fn get_all_contacts(
    query_parameters: HashMap<String, String>,
    contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    let pagination: Pagination = get_pagination(query_parameters)?;

    match contacts_repository
        .get_all(pagination.page_no, pagination.page_size)
        .await
    {
        Ok(val) => Ok(warp::reply::json(&val)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn get_contact(
    id: i32,
    contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    let possible_contact: Option<Contact> = match contacts_repository.get(ContactId(id)).await {
        Ok(x) => x,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    match possible_contact {
        Some(val) => Ok(warp::reply::json(&val)),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn add_conact(
    new_contact: NewContact,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    match contacts_repository.add(new_contact).await {
        Ok(val) => Ok(warp::reply::json(&val)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn update_contact(
    id: i32,
    contact: Contact,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    match contacts_repository.update(contact, ContactId(id)).await {
        Ok(val) => Ok(warp::reply::json(&val)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn delete_contact(
    id: i32,
    mut contacts_repository: impl ContactsRepository,
) -> Result<impl Reply, Rejection> {
    match contacts_repository.delete(ContactId(id)).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(warp::reject::custom(err)),
    }
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
            Err(error) => return Err(Error::StringToU32(error)),
        };
    }

    let mut page_size: Option<u32> = None;
    if query_parameters.contains_key(PAGE_SIZE) {
        page_size = match query_parameters.get(PAGE_SIZE).unwrap().parse::<u32>() {
            Ok(val) => Some(val),
            Err(error) => return Err(Error::StringToU32(error)),
        };
    }

    Ok(Pagination { page_no, page_size })
}

pub async fn handle_rejection(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
