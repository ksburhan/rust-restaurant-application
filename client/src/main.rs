use std::error::Error;
use std::{thread, time};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let tables = 100;
    let mut rng = rand::thread_rng();
    let mut loops = 1;
    loop{
        let table = rng.gen_range(0..tables);

        let add_items_request = format!("http://127.0.0.1:8000/api/v1/table/{}/items/", table);
        client.post(add_items_request)
            .body("[\"Pizza\", \"Burger\", \"Steak\",\"Sushi\"]")
            .send()
            .await?;

        thread::sleep(time::Duration::new(2, 0));

        let get_all_request1 = format!("http://127.0.0.1:8000/api/v1/table/{}", table);
        let resp2 = reqwest::get(get_all_request1)
            .await?
            .text()
            .await?;
        println!("{:#?}", resp2);
        
        thread::sleep(time::Duration::new(2, 0));

        let item_id = rng.gen_range(1..4) * loops;
        let get_all_of_one = format!("http://127.0.0.1:8000/api/v1/table/{}/items/{}", table, item_id);
        let resp3 = reqwest::get(get_all_of_one)
            .await?
            .text()
            .await?;
        println!("{:#?}", resp3);
        
        thread::sleep(time::Duration::new(2, 0));

        let remove_one = format!("http://127.0.0.1:8000/api/v1/table/{}/items/{}", table, item_id);
        client.delete(remove_one)
            .send()
            .await?;
            
        thread::sleep(time::Duration::new(2, 0));

        let get_all_request2 = format!("http://127.0.0.1:8000/api/v1/table/{}", table);
        let resp5 = reqwest::get(get_all_request2)
            .await?
            .text()
            .await?;
        println!("{:#?}", resp5);
        
        thread::sleep(time::Duration::new(2, 0));
        loops = loops + 1;
    }
}