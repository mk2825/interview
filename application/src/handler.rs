use crate::database::{get_connection, get_items_by_table_id, insert_order};
use crate::model::{Item, Order};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, PartialEq, Eq, Debug)]
pub(crate) struct ErrorPayload {
    msg: String,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum ResponsePayload<T> {
    Ok(T),
    Err(ErrorPayload),
}

impl<T: Serialize> IntoResponse for ResponsePayload<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ResponsePayload::Ok(v) => Json(v).into_response(),
            ResponsePayload::Err(e) => Json(e).into_response(),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct PostOrder {
    table_id: usize,
    item_ids: Vec<usize>,
}

pub(crate) async fn post_order(
    Json(payload): Json<PostOrder>,
) -> (StatusCode, ResponsePayload<Order>) {
    let order = Order {
        table_id: payload.table_id,
        item_ids: payload.item_ids,
    };

    let result = get_connection().and_then(|con| insert_order(&con, order));

    match result {
        Ok(order) => (StatusCode::OK, ResponsePayload::Ok(order)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponsePayload::Err(ErrorPayload {
                msg: format!("failed to create order, {e}"),
            }),
        ),
    }
}

pub(crate) async fn delete_order_by_table_id_and_item_id(
    Path((table_id, item_id)): Path<(usize, usize)>,
) -> (StatusCode, ResponsePayload<Value>) {
    let result = get_connection().and_then(|con| {
        crate::database::delete_order_by_table_id_and_item_id(&con, table_id, item_id)
    });

    match result {
        Ok(_) => (StatusCode::OK, ResponsePayload::Ok(json!({}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponsePayload::Err(ErrorPayload {
                msg: format!("failed to delete order, {e}"),
            }),
        ),
    }
}

pub(crate) async fn get_table_items_all(
    Path(table_id): Path<usize>,
) -> (StatusCode, ResponsePayload<Vec<Item>>) {
    let result = get_connection().and_then(|con| get_items_by_table_id(&con, table_id));

    match result {
        Ok(items) => (StatusCode::OK, ResponsePayload::Ok(items)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponsePayload::Err(ErrorPayload {
                msg: format!("failed to get items, {e}"),
            }),
        ),
    }
}

pub(crate) async fn get_table_item(
    Path((table_id, item_id)): Path<(usize, usize)>,
) -> (StatusCode, ResponsePayload<Value>) {
    let result =
        get_connection().and_then(|con| crate::database::get_table_item(&con, table_id, item_id));

    match result {
        Ok(item) => (StatusCode::OK, ResponsePayload::Ok(json!({"item": item}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponsePayload::Err(ErrorPayload {
                msg: format!("failed to get item, {e}"),
            }),
        ),
    }
}

#[cfg(test)]
mod test {
    use super::{post_order, PostOrder};
    use crate::{
        handler::{delete_order_by_table_id_and_item_id, get_table_items_all, ResponsePayload},
        model::{Item, Order},
    };
    use axum::{extract::Path, http::StatusCode, Json};
    use serde_json::json;
    use util::TestInitGuard;

    mod util {
        use crate::database::setup_database;
        use std::fs;

        pub(super) struct TestInitGuard {
            _private: (),
        }
        impl TestInitGuard {
            const TEST_DB_FILE_NAME: &'static str = "restaurant-test.db";

            pub(super) fn new() -> Self {
                Self::remove_file();
                setup_database(Self::TEST_DB_FILE_NAME).unwrap();
                Self { _private: () }
            }

            fn remove_file() {
                if fs::metadata(Self::TEST_DB_FILE_NAME).is_ok() {
                    fs::remove_file(Self::TEST_DB_FILE_NAME).unwrap();
                }
            }
        }
        impl Drop for TestInitGuard {
            fn drop(&mut self) {
                Self::remove_file();
            }
        }
    }

    #[tokio::test]
    async fn test_apis() {
        let _guard = TestInitGuard::new();

        // Insert a order (test for post api)
        let input = Json(PostOrder {
            table_id: 1,
            item_ids: vec![1, 2],
        });
        let (status, order) = post_order(input).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            order,
            ResponsePayload::Ok(Order {
                table_id: 1,
                item_ids: vec![1, 2]
            })
        );

        // Get all items (test for get api)
        let input = Path(1);
        let (status, items) = get_table_items_all(input).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            items,
            ResponsePayload::Ok(vec![
                Item {
                    id: 1,
                    name: "Burger".to_string()
                },
                Item {
                    id: 2,
                    name: "Pizza".to_string()
                },
            ])
        );

        // Delete one item (test for delete api)
        let input = Path((1, 1));
        let (status, order) = delete_order_by_table_id_and_item_id(input).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(order, ResponsePayload::Ok(json!({})));

        // Get all items (test for get api)
        let input = Path(1);
        let (status, items) = get_table_items_all(input).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            items,
            ResponsePayload::Ok(vec![Item {
                id: 2,
                name: "Pizza".to_string()
            },])
        );
    }
}
