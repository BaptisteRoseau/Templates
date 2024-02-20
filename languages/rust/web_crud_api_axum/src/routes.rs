use crate::{
    implementation::{create_item, delete_item, ping, read_item, update_item},
    models::Database,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Defines the routes for CRUD operations
#[allow(opaque_hidden_inferred_bound)]
pub(crate) fn routes() -> Router {
    let database: Database = Arc::new(RwLock::new(HashMap::new()));
    let path_item = "/v1/item";
    let path_item_id = "/v1/item/:id";
    Router::new()
        .route("/ping", get(ping))
        .route(path_item, post(create_item))
        .route(path_item_id, get(read_item))
        .route(path_item_id, put(update_item))
        .route(path_item_id, delete(delete_item))
        .with_state(database)
}
