#[macro_use]
extern crate diesel;

// Modules
mod db;
mod models;
mod ops;
mod schema;
mod scraping;
mod structs;

#[tokio::main]
async fn main() {
    const UPDATE: bool = true;
    
    if UPDATE {
        let boulders = match scraping::fetch_boulders().await {
            Ok(boulders) => boulders,
            Err(e) => panic!("Error fetching boulders! Details: {}", e)
        };
    }

}
