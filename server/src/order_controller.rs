#[post("/table/<table_id>/items")]
pub fn add_items(
  table_id: i64,
){
  println!("post")
}

#[delete("/table/<table_id>/item/<item_id>")]
pub fn delete_item(
  table_id: i64,
  item_id: i64,
){
  println!("delete")
}

#[get("/table/<table_id>/items")]
pub fn get_all_table_items(
  table_id: i64,
){
  println!("get 1")
}

#[get("/table/<table_id>/item/<item_id>")]
pub fn get_table_item(
  table_id: i64,
  item_id: i64,
){
  println!("get 2")
}