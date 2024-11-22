use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct Item {
    pub(crate) id: usize,
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct Order {
    pub(crate) table_id: usize,
    pub(crate) item_ids: Vec<usize>,
}

// This isn't actually used for now.
#[derive(Deserialize)]
pub(crate) struct Table {
    pub(crate) _id: usize,
}
