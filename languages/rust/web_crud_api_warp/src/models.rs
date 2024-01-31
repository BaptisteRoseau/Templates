use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// The basic items stored in the database
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct Item {
    pub name: String,
    pub content: String,
}

impl Item {
    pub fn update(&mut self, item: &Item) {
        self.name = item.name.clone();
        self.content = item.content.clone();
    }
}

/// The database containing all data accessed by the API
pub(crate) type Database = Arc<RwLock<HashMap<Uuid, Item>>>;

// =================================================================================================
// TESTS
// =================================================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_update() {
        let mut item_1 = Item {
            name: "a".to_string(),
            content: "b".to_string(),
        };
        let item_2 = Item {
            name: "c".to_string(),
            content: "d".to_string(),
        };
        item_1.update(&item_2);

        assert_eq!(item_1, item_2);
    }
}
