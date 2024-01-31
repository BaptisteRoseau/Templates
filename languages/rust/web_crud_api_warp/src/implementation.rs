use crate::models::{Database, Item};
use uuid::Uuid;
use warp::{Rejection, Reply};

/// Health Check of the API
pub(crate) async fn ping() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::reply())
}

/// Creates a new item an returns its ID
pub(crate) async fn create_item(
    new_item: Item,
    database: Database,
) -> Result<impl Reply, Rejection> {
    let uuid = Uuid::new_v4();
    let mut guard = database.write();
    guard.insert(uuid, new_item);
    Ok(warp::reply::json(&uuid))
}

/// Get an item from the database
pub(crate) async fn read_item(id: Uuid, database: Database) -> Result<impl Reply, Rejection> {
    let guard = database.read();
    match guard.get(&id) {
        Some(item) => Ok(warp::reply::json(&item)),
        None => Err(warp::reject::not_found()),
    }
}

/// Creates an existing item
pub(crate) async fn update_item(
    id: Uuid,
    updated_item: Item,
    database: Database,
) -> Result<impl Reply, Rejection> {
    let mut guard = database.write();
    match guard.get_mut(&id) {
        Some(item) => {
            item.update(&updated_item);
            Ok::<_, warp::Rejection>(warp::reply::json(&item))
        }
        None => Err(warp::reject::not_found()),
    }
}

/// Delete an existing item
pub(crate) async fn delete_item(id: Uuid, database: Database) -> Result<impl Reply, Rejection> {
    let mut guard = database.write();
    match guard.remove(&id) {
        Some(_) => Ok::<_, warp::Rejection>(warp::reply()),
        None => Err(warp::reject::not_found()),
    }
}

// =================================================================================================
// TESTS
// =================================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use parking_lot::RwLock;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::Arc;
    use warp::http::Response;
    use warp::hyper::body::to_bytes;
    use warp::hyper::Body;

    // Data

    fn item_1() -> Item {
        Item {
            name: "name".to_string(),
            content: "content".to_string(),
        }
    }

    #[allow(dead_code)] // Remove when used
    fn item_2() -> Item {
        Item {
            name: "another name".to_string(),
            content: "another content".to_string(),
        }
    }

    fn database() -> Database {
        Arc::new(RwLock::new(HashMap::new()))
    }

    // Helpers

    async fn response_to_uuid(response: Response<Body>) -> Uuid {
        Uuid::from_str(
            String::from_utf8(to_bytes(response.into_body()).await.unwrap().to_vec())
                .unwrap()
                .trim_matches('\"'),
        )
        .unwrap()
    }

    async fn body_to_string(body: Body) -> String {
        String::from_utf8(to_bytes(body).await.unwrap().to_vec())
            .unwrap()
            .trim_matches('\"')
            .to_string()
    }

    async fn body_to_item(body: Body) -> Item {
        serde_json::from_str(body_to_string(body).await.as_str().trim_matches('\"')).unwrap()
    }

    async fn create_item_and_get_id(item: Item, database: Database) -> Uuid {
        response_to_uuid(
            create_item(item.clone(), database.clone())
                .await
                .unwrap()
                .into_response(),
        )
        .await
    }

    // Tests

    #[tokio::test]
    async fn create_item_returns_id() {
        let database = database();
        let item = item_1();

        let response = create_item(item, database).await.unwrap().into_response();
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn read_item_present() {
        let database = database();
        let item = item_1();

        // First Item
        let id: Uuid = create_item_and_get_id(item.clone(), database.clone()).await;
        let response = read_item(id, database.clone())
            .await
            .unwrap()
            .into_response();

        assert!(response.status().is_success());
        assert_eq!(body_to_item(response.into_body()).await, item);

        // Second Item
        let id: Uuid = create_item_and_get_id(item.clone(), database.clone()).await;
        let response = read_item(id, database.clone())
            .await
            .unwrap()
            .into_response();

        assert!(response.status().is_success());
        assert_eq!(body_to_item(response.into_body()).await, item);
    }

    #[tokio::test]
    async fn read_item_absent() {
        let database = database();
        let id: Uuid = Uuid::new_v4();
        let reply = read_item(id, database).await;
        assert!(reply.is_err());
    }

    #[tokio::test]
    async fn update_item_present() {
        let database = database();
        let item_1 = item_1();
        let item_2 = item_2();
        let id: Uuid = create_item_and_get_id(item_1.clone(), database.clone()).await;
        let response = update_item(id, item_2.clone(), database.clone())
            .await
            .unwrap()
            .into_response();

        assert!(response.status().is_success());

        let response = read_item(id, database.clone())
            .await
            .unwrap()
            .into_response();

        assert!(response.status().is_success());
        assert_eq!(body_to_item(response.into_body()).await, item_2);
    }

    #[tokio::test]
    async fn update_item_absent() {
        let database = database();
        let item = item_1();
        let id: Uuid = Uuid::new_v4();
        let reply = update_item(id, item, database).await;
        assert!(reply.is_err());
    }

    #[tokio::test]
    async fn delete_item_present() {
        let database = database();
        let item = item_1();
        let id: Uuid = create_item_and_get_id(item.clone(), database.clone()).await;
        let reply = delete_item(id, database).await;
        assert!(reply.is_ok());
    }

    #[tokio::test]
    async fn delete_item_absent() {
        let database = database();
        let id: Uuid = Uuid::new_v4();
        let reply = delete_item(id, database).await;
        assert!(reply.is_err());
    }
}
