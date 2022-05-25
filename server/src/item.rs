
use serde::Serialize;
#[derive(Serialize, Clone)]
pub struct Item{
    pub id: i64,
    pub name: String,
    pub table_id: i64,
    pub finishes_at: i64,
}