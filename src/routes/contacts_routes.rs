use std::convert::Infallible;

use warp::hyper::Method;
use warp::Filter;

use crate::handlers::contacts_handlers::get_all_contacts;
use crate::handlers::contacts_handlers::handle_rejection;
use crate::repository::contacts_db_repository::ContactsDbRepository;

pub fn get_all_routes(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([
            Method::GET.as_str(),
            Method::POST.as_str(),
            Method::PUT.as_str(),
            Method::DELETE.as_str(),
            Method::OPTIONS.as_str(),
        ]);

    let routes = get_all_questions_route(db_repository.clone())
        .with(cors)
        .recover(handle_rejection);

    return routes;
}

fn get_all_questions_route(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("contacts")
        .and(warp::get())
        .and(warp::query())
        .and(with_repository(db_repository))
        .and_then(get_all_contacts)
}

fn with_repository(
    db_repository: ContactsDbRepository,
) -> impl Filter<Extract = (ContactsDbRepository,), Error = Infallible> + Clone {
    warp::any().map(move || db_repository.clone())
}
