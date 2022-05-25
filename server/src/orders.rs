use crate::{item::Item};
use std::collections::hash_map::HashMap;

pub struct Orders{
    pub order: HashMap<i64, Item>
}

impl Orders{
    pub fn new() -> Orders{
        Orders{
            order: HashMap::new()
        }
    }
}