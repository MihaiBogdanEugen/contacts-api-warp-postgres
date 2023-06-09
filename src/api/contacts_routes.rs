use std::convert::Infallible;

use serde::de::DeserializeOwned;
use warp::cors::Builder;
use warp::hyper::Method;
use warp::log::Info;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

use crate::api::contacts_handlers;
use crate::repositories::contacts_db_repository::ContactsDbRepository;

const MAX_JSON_PAYLOAD_SIZE: u64 = 1024 * 16;

pub fn get_all_routes(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cors: Builder = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([
            Method::GET.as_str(),
            Method::POST.as_str(),
            Method::PUT.as_str(),
            Method::DELETE.as_str(),
            Method::OPTIONS.as_str(),
        ]);

    let logging = warp::log::custom(|info: Info| {
        eprintln!("{} {} {}", info.method(), info.path(), info.status());
    });

    get_all_contacts_route(db_repository.clone())
        .or(get_contact_route(db_repository.clone()))
        .or(add_contact_route(db_repository.clone()))
        .or(update_contact_route(db_repository.clone()))
        .or(update_contact_email_route(db_repository.clone()))
        .or(update_contact_phone_no_route(db_repository.clone()))
        .or(delete_contact_route(db_repository))
        .with(cors)
        .with(logging)
        .recover(contacts_handlers::handle_rejection)
}

fn get_all_contacts_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts")
        .and(warp::get())
        .and(warp::query())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::get_all_contacts)
}

fn get_contact_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts" / i32)
        .and(warp::get())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::get_contact)
}

fn add_contact_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts")
        .and(warp::post())
        .and(json_body())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::add_conact)
}

fn update_contact_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts" / i32)
        .and(warp::put())
        .and(json_body())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::update_contact)
}

fn update_contact_email_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts-update-email" / i32)
        .and(warp::post())
        .and(json_body())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::update_contact_email)
}

fn update_contact_phone_no_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts-update-phone-no" / i32)
        .and(warp::post())
        .and(json_body())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::update_contact_phone_no)
}

fn delete_contact_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contacts" / i32)
        .and(warp::delete())
        .and(with_repository(db_repository))
        .and_then(contacts_handlers::delete_contact)
}

fn with_repository(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (ContactsDbRepository,), Error = Infallible> + Clone {
    warp::any().map(move || db_repository.clone())
}

fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
{
    warp::body::content_length_limit(MAX_JSON_PAYLOAD_SIZE).and(warp::body::json())
}
