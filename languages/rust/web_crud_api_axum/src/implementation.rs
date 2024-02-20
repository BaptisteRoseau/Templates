use crate::models::{Database, Item};
use axum;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Health Check of the API
///
pub(crate) async fn ping() -> &'static str {
    "OK"
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ResponseUuid {
    id: Uuid,
}

/// Creates a new item an returns its ID
pub(crate) async fn create_item(
    State(database): State<Database>,
    axum::extract::Json(new_item): axum::extract::Json<Item>,
) -> Result<(StatusCode, axum::response::Json<ResponseUuid>), StatusCode> {
    let uuid = Uuid::new_v4();
    let mut guard = database.write();
    match guard.insert(uuid, new_item) {
        Some(_) => Ok((StatusCode::CREATED, axum::Json(ResponseUuid { id: uuid }))),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get an item from the database
pub(crate) async fn read_item(
    State(database): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<axum::response::Json<Item>, StatusCode> {
    let guard = database.read();
    match guard.get(&id) {
        Some(item) => Ok(axum::Json(item.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Creates an existing item
pub(crate) async fn update_item(
    State(database): State<Database>,
    Path(id): Path<Uuid>,
    axum::extract::Json(updated_item): axum::extract::Json<Item>,
) -> Result<axum::response::Json<Item>, StatusCode> {
    let mut guard = database.write();
    match guard.get_mut(&id) {
        Some(item) => {
            item.update(&updated_item);
            Ok(axum::Json(item.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete an existing item
pub(crate) async fn delete_item(
    State(database): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut guard = database.write();
    match guard.remove(&id) {
        Some(_) => Ok(StatusCode::OK),
        None => Err(StatusCode::NOT_FOUND),
    }
}
