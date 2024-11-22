# Application
This is a implementation of SimpleRestaurantApi described in `SimpleRestaurantApi.md`.

# Build, Run
```bash
# build
cargo build --release -p application

# run
cargo run --release -p application
```

# Test
```bash
cargo test -p application
```

# API Endpoints Specification

## 1. `POST /order`
Register items to the specific table.
### Request
- **Method:** `POST`
- **Body:**
  - `table_id` (integer): The ID of the target table.
  - `item_ids` (array of integers): A list of item IDs to order.

### Response
- **Status Code:** `200 OK`  
  - **Body:**
    - `table_id` (integer): The ID of the target table.
    - `item_ids` (array of integers): The list of item IDs successfully added to the order.
- **Status Code:** `500 Internal Server Error`  
  - **Body:**
    - `msg` (string): An error message detailing the failure.

---

## 2. `DELETE /order/{table_id}/{item_id}`
Delete a item from the specific table.
### Request
- **Method:** `DELETE`
- **Path Parameters:**
  - `table_id` (integer): The ID of the target table.
  - `item_id` (integer): The ID of the item to delete from the order.

### Response
- **Status Code:** `200 OK`  
  - **Body:** Empty object.
- **Status Code:** `500 Internal Server Error`  
  - **Body:**
    - `msg` (string): An error message detailing the failure.

---

## 3. `GET /table/{table_id}/items`
Get all items from the specific table.
### Request
- **Method:** `GET`
- **Path Parameters:**
  - `table_id` (integer): The ID of the target table.

### Response
- **Status Code:** `200 OK`  
  - **Body:** A list of items, where each item contains:
    - `id` (integer): The ID of the item.
    - `name` (string): The name of the item.
- **Status Code:** `500 Internal Server Error`  
  - **Body:**
    - `msg` (string): An error message detailing the failure.

---

## 4. `GET /table/{table_id}/item/{item_id}`
Get a item from the specific table.
### Request
- **Method:** `GET`
- **Path Parameters:**
  - `table_id` (integer): The ID of the target table.
  - `item_id` (integer): The ID of the specific item to retrieve.

### Response
- **Status Code:** `200 OK`  
  - **Body:** An object containing:
    - `item` (object):
      - `id` (integer): The ID of the item.
      - `name` (string): The name of the item.
- **Status Code:** `500 Internal Server Error`  
  - **Body:**
    - `msg` (string): An error message detailing the failure.


# Request Sample

```bash
# create order
curl -XPOST -H "Content-Type: application/json" -d '{"table_id": 1, "item_ids": [1,2]}'  'http://localhost:3000/order'

# delete order
curl -XDELETE -H "Content-Type: application/json" 'http://localhost:3000/order/1/2'

# get all items for specific table
curl -XGET -H "Content-Type: application/json" 'http://localhost:3000/order/1'

# get all item for specific table and item_id
curl -XGET -H "Content-Type: application/json" 'http://localhost:3000/order/1/2'

```

# Storage
I use sqlite as a storage. We can check the contents stored by sqlite by using sqlite cli:

1. **Install SQLite CLI**:
   - On macOS:
     ```bash
     brew install sqlite
     ```
   - On Linux:
     ```bash
     sudo apt install sqlite3
     ```

2. **Open the Database**:
   ```bash
   sqlite3 orders.db
   ```

3. **View Tables**:
   ```sql
   .tables
   ```

4. **Inspect Table Schema**:
   ```sql
   .schema
   ```

5. **Query Data**:
   ```sql
   SELECT * FROM `Orders`;
   ```

6. **Exit**:
   ```bash
   .exit
   ```

# Schema

### items
| Column | Type    | Constraints |
|--------|---------|-------------|
| id     | INTEGER | PRIMARY KEY |
| name   | TEXT    | NOT NULL    |

### tables
| Column | Type    | Constraints |
|--------|---------|-------------|
| id     | INTEGER | PRIMARY KEY |

### orders
| Column         | Type    | Constraints                                                                 |
|----------------|---------|-----------------------------------------------------------------------------|
| table_id       | INTEGER | NOT NULL, FOREIGN KEY (table_id) REFERENCES `tables`(id)                   |
| item_id        | INTEGER | NOT NULL, FOREIGN KEY (item_id) REFERENCES `items`(id)                     |
| cook_duration  | INTEGER | NOT NULL                                                                  |

### Indexes

| Table   | Index Name               | Columns            |
|---------|--------------------------|--------------------|
| orders  | idx_orders_table_id      | table_id           |
| orders  | idx_orders_table_item    | table_id, item_id  |
