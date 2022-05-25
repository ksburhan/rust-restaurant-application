use crate::{order_service::{OrderService}, item::Item};
use rocket::{
  {post, get, delete},
  http::Status,
  serde::json::Json,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref ORDER_SERVICE: OrderService = OrderService::new();
}

#[post("/table/<table_id>/items")]
pub fn add_items(
  table_id: i64,
) -> Status {
  (ORDER_SERVICE.add_items(table_id));
  Status::Ok
}

#[delete("/table/<table_id>/item/<item_id>")]
pub fn delete_item(
  table_id: i64,
  item_id: i64,
) -> Status {
  ORDER_SERVICE.delete_item(table_id, item_id);
  Status::Ok
}

#[get("/table/<table_id>/items")]
pub fn get_all_table_items(
  table_id: i64,
) -> Status {
  ORDER_SERVICE.get_all_table_items(table_id);
  Status::Ok
}

#[get("/table/<table_id>/item/<item_id>")]
pub fn get_table_item(
  table_id: i64,
  item_id: i64,
) -> Status {
  ORDER_SERVICE.get_table_item(table_id, item_id);
  Status::Ok
}