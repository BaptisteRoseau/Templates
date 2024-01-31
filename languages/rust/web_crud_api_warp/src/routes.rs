use crate::implementation::{create_item, delete_item, ping, read_item, update_item};
use crate::models::Database;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

/// Defines the routes for CRUD operations
#[allow(opaque_hidden_inferred_bound)]
pub(crate) fn routes(
    database: Database,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let ping = warp::path!("ping").and(warp::get()).and_then(ping);

    let get_item = warp::path!("v1" / "items" / Uuid)
        .and(warp::get())
        .and(with_database(database.clone()))
        .and_then(read_item);

    let create_item = warp::path!("v1" / "items")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_database(database.clone()))
        .and_then(create_item);

    let update_item = warp::path!("v1" / "items" / Uuid)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_database(database.clone()))
        .and_then(update_item);

    let delete_item = warp::path!("v1" / "items" / Uuid)
        .and(warp::delete())
        .and(with_database(database))
        .and_then(delete_item);

    ping.or(get_item)
        .or(create_item)
        .or(update_item)
        .or(delete_item)
}

/// Helper function to pass the database as a parameter
fn with_database(
    database: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || database.clone())
}
