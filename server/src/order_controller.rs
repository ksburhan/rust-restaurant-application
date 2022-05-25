use crate::{order_service::{OrderService}, item::Item};

use rocket::{
  {post, get, delete},
  serde::json::Json,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref ORDER_SERVICE: OrderService = OrderService::new(100, 500);
}

fn check_result1(result: Result<Vec<Item>, String>) -> Result<Json<Vec<Item>>, String> {
  match result{
    Ok(result) => Ok(Json(result)),
    Err(result) => Err(result)
  }
}

fn check_result2(result: Result<(), String>) -> Result<Json<()>, String> {
  match result{
    Ok(result) => Ok(Json(result)),
    Err(result) => Err(result)
  }
}

#[post("/table/<table_id>/items", data = "<items>")]
pub fn add_items(
  table_id: i64,
  items: Json<Vec<String>>
) -> Result<Json<Vec<Item>>, String> {
  check_result1(ORDER_SERVICE.add_items(table_id, &items))
}

#[delete("/table/<table_id>/items/<item_id>")]
pub fn delete_item(
  table_id: i64,
  item_id: i64,
) -> Result<Json<()>, String> {
  check_result2(ORDER_SERVICE.delete_item(table_id, item_id))
}

#[get("/table/<table_id>")]
pub fn get_all_table_items(
  table_id: i64,
) -> Result<Json<Vec<Item>>, String> {
  check_result1(ORDER_SERVICE.get_all_table_items(table_id))
}

#[get("/table/<table_id>/items/<item_id>")]
pub fn get_table_item(
  table_id: i64,
  item_id: i64,
) -> Result<Json<Vec<Item>>, String> {
  check_result1(ORDER_SERVICE.get_table_item(table_id, item_id))
}