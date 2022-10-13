use crate::models::Boulder;

use crate::schema::app_users::id;
use crate::schema::boulders::setter_two_id;
use crate::scraping::boulder_object::BoulderObject;
use crate::ops::{create_setter_user, get_setter_by, create_boulder};
use crate::structs::BoulderData;

use dotenv::dotenv;
use std::env;

use super::fetch_setter;

// use super::boulder_object::BoulderObject;

pub async fn fetch_boulders() -> Result<Vec<Boulder>, reqwest::Error> {
    dotenv().ok();

    // Get URLs from environment
    let url_boulderlist = match env::var("URL_BOULDERLIST") {
        Ok(val) => val,
        Err(_) => String::from("https://api-mn.boulderhalle.app/collection/activeBoulders")
    };
    let url_boulder = match env::var("URL_BOULDER") {
        Ok(val) => val,
        Err(_) => String::from("https://api-mn.boulderhalle.app/objects/")
    };

    // Create reqwest client
    let client = reqwest::Client::new();

    // Get All Boulder IDs from official boulder api
    let response = client.get(url_boulderlist).send().await?;

    let response_json = response.json::<Vec<String>>().await?;
    
    // Initialize Vector to store all boulders
    let mut fetched: Vec<Boulder> = Vec::new();

    // Fetch all boulders
    for boulder_id in response_json.clone() {
        let url = format!("{}{}", url_boulder, boulder_id);

        let boulderobj: BoulderObject = reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        let setter_id = get_setter_by(boulderobj.content.setter);

        let creation_data = match setter_id {
            Some(id_i32) => {
                BoulderData::new(boulderobj, id_i32, None)
            },
            None => {
                let res = fetch_setter(boulderobj.content.setter).expect("Could not fetch setter");
                BoulderData::new(boulderobj, res[0], res[1])
            }
        };

        create_boulder(creation_data);
    }


    todo!("Return")
}