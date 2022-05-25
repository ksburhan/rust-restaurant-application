use crate::item::Item;

pub struct OrderService {
    all_tables: i64,
}

impl OrderService{
    pub fn new(

    ) -> OrderService {
        let all_tables = 4;
        OrderService {
            all_tables,
        }
    }

    pub fn add_items(&self, table_id: i64) -> Result<Vec<Item>, String> {
        println!("item added");
        Ok(Vec::new())
    }

    pub fn delete_item(&self, table_id: i64, item_id: i64) -> Result<(), String> {
        println!("deleted");
        Ok(())
    }

    pub fn get_all_table_items(&self, table_id: i64) -> Result<Vec<Item>, String> {
        println!("item all table shown");
        Ok(Vec::new())
    }

    pub fn get_table_item(&self, table_id: i64, item_id: i64) -> Result<Item, String>  {
        println!("table item shown");
        Ok(Item{
            id: item_id,
            name: String::from("asd"),
            table_id: table_id,
            starts_at: 123,
            finishes_at: 123,
            is_done: true
        })
    }
}