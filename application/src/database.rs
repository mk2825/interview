use crate::model::{Item as ItemModel, Order as OrderModel};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize)]
pub(crate) struct Item {
    pub(crate) id: usize,
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Order {
    pub(crate) table_id: usize,
    pub(crate) item_id: usize,
    pub(crate) cook_duration: usize,
}

// Returns 5 ~ 15
fn random_dur() -> u64 {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    time.as_secs() % 10 + 5
}

pub(crate) fn get_connection() -> rusqlite::Result<Connection> {
    #[cfg(not(test))]
    let con = Connection::open("restaurant.db");
    #[cfg(test)]
    let con = Connection::open("restaurant-test.db");

    con
}

// This function does following things:
//
// * Create tables
// * Create indexes
// * Insert master datas
pub(crate) fn setup_database<P: AsRef<Path>>(db_name: P) -> rusqlite::Result<()> {
    let conn = Connection::open(db_name)?;

    conn.execute("BEGIN TRANSACTION", [])?;

    // Once we get the error, abort the transaction.

    let res = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
            [],
        )
        .and_then(|_| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS tables (
             id INTEGER PRIMARY KEY
            )",
                [],
            )
        })
        .and_then(|_| {
            conn.execute(
                "
                CREATE TABLE IF NOT EXISTS orders (
                    table_id INTEGER NOT NULL,
                    item_id INTEGER NOT NULL,
                    cook_duration INTEGER NOT NULL,
                    FOREIGN KEY (table_id) REFERENCES `Table`(id),
                    FOREIGN KEY (item_id) REFERENCES Item(id)
                )",
                [],
            )
        })
        .and_then(|_| {
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_orders_table_id ON `orders`(table_id)",
                [],
            )
        })
        .and_then(|_| {
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_orders_table_item ON `orders`(table_id, item_id)",
                [],
            )
        })
        .and_then(|_| {
            // Insert master data
            let tables = vec![1, 2, 3];
            let res: rusqlite::Result<Vec<usize>> = tables
                .into_iter()
                .map(|table_id| {
                    conn.execute(
                        "INSERT OR IGNORE INTO `Tables` (id) VALUES (?1)",
                        params![table_id],
                    )
                })
                .collect();

            res
        })
        .and_then(|_| {
            // Insert master data
            let items = vec![(1, "Burger"), (2, "Pizza"), (3, "Pasta"), (4, "Salad")];
            let res: rusqlite::Result<Vec<usize>> = items
                .into_iter()
                .map(|(item_id, item_name)| {
                    conn.execute(
                        "INSERT OR IGNORE INTO Items (id, name) VALUES (?1, ?2)",
                        params![item_id, item_name],
                    )
                })
                .collect();

            res
        });

    match res {
        Ok(_) => {
            conn.execute("COMMIT TRANSACTION", [])?;
            Ok(())
        }
        Err(e) => {
            conn.execute("ROLLBACK TRANSACTION", [])?;
            Err(e)
        }
    }
}

pub(crate) fn insert_order(conn: &Connection, order: OrderModel) -> rusqlite::Result<OrderModel> {
    // Start transaction
    conn.execute("BEGIN TRANSACTION", [])?;

    for item_id in &order.item_ids {
        // Randomly generate a cooking time
        let cook_dur = random_dur();

        match conn.execute(
            "INSERT INTO orders (table_id, item_id, cook_duration) VALUES (?1, ?2, ?3)",
            params![order.table_id, item_id, cook_dur],
        ) {
            Ok(_) => continue,
            Err(e) => {
                // Abort the transaction!
                conn.execute("ROLLBACK TRANSACTION", [])?;

                return Err(e);
            }
        }
    }

    // Commit the transaction
    conn.execute("COMMIT TRANSACTION", [])?;

    Ok(order)
}

pub(crate) fn get_items_by_table_id(
    conn: &Connection,
    table_id: usize,
) -> rusqlite::Result<Vec<ItemModel>> {
    let mut stmt =
        conn.prepare("
        SELECT item_id, name FROM orders INNER JOIN items ON items.id = orders.item_id WHERE table_id = :table_id"
    )?;
    let mut items: Vec<ItemModel> = Vec::new();

    let rows = stmt.query_map([table_id], |row| {
        Ok((row.get::<_, usize>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (item_id, name) = row?;
        let item = ItemModel { id: item_id, name };
        items.push(item);
    }

    Ok(items)
}

pub(crate) fn get_table_item(
    conn: &Connection,
    table_id: usize,
    item_id: usize,
) -> rusqlite::Result<Option<ItemModel>> {
    let mut stmt = conn.prepare(
        "SELECT table_id, name FROM orders INNER JOIN items ON items.id = orders.item_id WHERE table_id = :table_id AND item_id = :item_id",
    )?;
    let mut items: Vec<ItemModel> = Vec::new();

    let rows = stmt.query_map([table_id, item_id], |row| {
        Ok((row.get::<_, usize>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (item_id, name) = row?;
        let item = ItemModel { id: item_id, name };
        items.push(item);
    }

    let item = items.pop();

    Ok(item)
}

pub(crate) fn delete_order_by_table_id_and_item_id(
    conn: &Connection,
    table_id: usize,
    item_id: usize,
) -> rusqlite::Result<()> {
    let mut stmt =
        conn.prepare("DELETE FROM orders WHERE table_id = :table_id AND item_id = :item_id")?;

    stmt.execute(params![table_id, item_id])?;

    Ok(())
}
