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

    //pub fn remove_finished_food(&self, table_id: i64, item_id: i64){
    //    let target_table = &self.all_tables[table_id as usize];
    //    let mut target_table_orders = target_table.lock().unwrap();
    //    if let Some(_) = target_table_orders.order.remove(&item_id){
    //    } else {
    //    }
    //}
    pub fn add_items(&self, table_id: i64, items: &Vec<String>) -> Result<Vec<Item>, String> {
        if &table_id >= &self.max_tables {
            return Err(String::from("Not a valid table_id!"))
        }
        let target_table = &self.all_tables[table_id as usize];
        let mut target_table_orders = target_table.lock().unwrap();

        //for item in target_table_orders.order.iter() {
        //    if item.1.finishes_at <= Utc::now().timestamp() {
        //        let id = item.1.id.clone() as i64;
        //        self.remove_finished_food(table_id, id);
        //    }
        //}

        if (target_table_orders.order.len() + items.len()) as i64 > self.max_items {
            return Err(String::from("Already too many items!"))
        }

        let mut added_items = Vec::new();
        let mut rng = rand::thread_rng();
        
        for item in items {
            let i = Item{
                id: (target_table_orders.order.len() + 1) as i64,
                name: item.to_string(),
                table_id: table_id,
                finishes_at: Utc::now().timestamp() + rng.gen_range(5..15),
            };
            added_items.push(i.clone());
            target_table_orders.order.insert(i.id, i.clone());
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
        if &table_id >= &self.max_tables {
            return Err(String::from("Not a valid table_id!"))
        }
        let target_table = &self.all_tables[table_id as usize];
        let target_table_orders = target_table.lock().unwrap();
        Ok(target_table_orders.order.values().cloned().collect())
    }

    pub fn get_table_item(&self, table_id: i64, item_id: i64) -> Result<Vec<Item>, String>  {
        if &table_id >= &self.max_tables {
            return Err(String::from("Not a valid table_id!"))
        }
        let target_table = &self.all_tables[table_id as usize];
        let target_table_orders = target_table.lock().unwrap();
        if let Some(i) = target_table_orders.order.get(&item_id){
            Ok(vec![i.clone()])
        } else {
            Err(String::from("Item not found on table!"))
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_item(id: i64, name: &str, table_id: i64, finishes_at: i64) -> Item {
    Item {
      id: id,
      name: name.to_string(),
      table_id: table_id,
      finishes_at,
    }
  }

  #[test]
  fn test_add_item() {
    let item1 = create_item(1, "Pizza", 0, 60);
    let item2 = create_item(2, "Steak", 0, 60);
    let item3 = create_item(3, "Burger", 0, 60);
    let item4 = create_item(4, "Sushi", 0, 60);

    let mut orders = Orders::new();
    assert_eq!(0, orders.order.len());

    for (i, x) in vec![item1, item2, item3, item4].iter().enumerate() {
        orders.order.insert(i as i64, x.clone());
      assert_eq!(i + 1, orders.order.len());
    }
  }

  #[test]
  fn test_get_item() {
    let item1 = create_item(1, "Pizza", 0, 60);
    let item2 = create_item(2, "Steak", 0, 60);
    let item3 = create_item(3, "Burger", 0, 60);
    let item4 = create_item(4, "Sushi", 0, 60);
    
    let items = vec![item1, item2, item3, item4];

    let mut orders = Orders::new();
    for (i, x) in items.iter().enumerate() {
        orders.order.insert(i as i64, x.clone());
      assert_eq!(i + 1, orders.order.len());
    }
  }
}
