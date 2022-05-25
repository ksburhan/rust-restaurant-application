use crate::item::Item;
use crate::orders::Orders;
use std::sync::Mutex;
use chrono::Utc;
use rand::Rng;

macro_rules! vec_no_clone {
    ($val:expr; $n:expr) => {{
      std::iter::repeat_with(|| $val).take($n as usize).collect()
    }};
  }
  

pub struct OrderService {
    max_tables: i64,
    max_items: i64,
    all_tables: Vec<Mutex<Orders>>,
}

impl OrderService{
    pub fn new(
        max_tables: i64,
        max_items: i64,
    ) -> OrderService {
        OrderService {
            max_tables: max_tables,
            max_items: max_items,
            all_tables: vec_no_clone![Mutex::new(Orders::new()); max_tables],
        }
    }

    pub fn add_items(&self, table_id: i64, items: &Vec<String>) -> Result<Vec<Item>, String> {
        if &table_id >= &self.max_tables {
            return Err(String::from("Not a valid table_id!"))
        }
        let target_table = &self.all_tables[table_id as usize];
        let mut target_table_orders = target_table.lock().unwrap();
        if (target_table_orders.order.len() + items.len()) as i64 > self.max_items {
            return Err(String::from("Already too many items!"))
        }

        let mut added_items = Vec::new();
        let mut rng = rand::thread_rng();
        
        let mut idd = 1;
        for item in items {
            let i = Item{
                id: idd,
                name: item.to_string(),
                table_id: table_id,
                starts_at: Utc::now().timestamp(),
                finishes_at: Utc::now().timestamp() + rng.gen_range(5000..15000),
                is_done: false,
            };
            added_items.push(i.clone());
            target_table_orders.order.insert(i.id, i.clone());
            idd = idd +1;
        }
        Ok(added_items)
    }

    pub fn delete_item(&self, table_id: i64, item_id: i64) -> Result<(), String> {
        if &table_id >= &self.max_tables {
            return Err(String::from("Not a valid table_id!"))
        }
        
        let target_table = &self.all_tables[table_id as usize];
        let mut target_table_orders = target_table.lock().unwrap();

        if let Some(_) = target_table_orders.order.remove(&item_id){
            Ok(())
        } else {
            Err(String::from("Item not found on table!"))
        }
    }

    pub fn get_all_table_items(&self, table_id: i64) -> Result<Vec<Item>, String> {
        println!("item all table shown");
        Ok(Vec::new())
    }

    pub fn get_table_item(&self, table_id: i64, item_id: i64) -> Result<Vec<Item>, String>  {
        println!("table item shown");
        Ok(vec![Item{
            id: item_id,
            name: String::from("asd"),
            table_id: table_id,
            starts_at: 123,
            finishes_at: 123,
            is_done: true
        }])
    }
}