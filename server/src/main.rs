#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use server::{
  order_controller::{
    add_items,
    delete_item,
    get_all_table_items,
    get_table_item}};


#[launch]
fn rocket() -> _ {
       rocket::build()
       .mount(
         "/api/v1", 
         routes![
          add_items,
          delete_item,
          get_all_table_items,
          get_table_item
           ])
}