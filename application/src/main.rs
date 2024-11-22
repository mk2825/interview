mod database;
mod handler;
mod model;

use axum::{
    routing::{delete, get, post},
    Router,
};
use database::setup_database;
use handler::{
    delete_order_by_table_id_and_item_id, get_table_item, get_table_items_all, post_order,
};

#[tokio::main]
async fn main() {
    setup_database("restaurant.db")
        .map_err(|e| eprintln!("failed to setup database, {e}"))
        .unwrap();

    let app = Router::new()
        .route("/order", post(post_order))
        .route(
            "/order/:table_id/:item_id",
            delete(delete_order_by_table_id_and_item_id),
        )
        .route("/order/:table_id", get(get_table_items_all))
        .route("/order/:table_id/:item_id", get(get_table_item));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
