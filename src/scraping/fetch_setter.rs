use crate::{structs::SetterData, ops::create_setter_user};

use super::SetterObject;

use dotenv::dotenv;
use std::env;

pub async fn fetch_setter(setters: Vec<String>) -> Result<SetterReturnValue, reqwest::Error> {
    dotenv().ok();
    let url_setter = match env::var("URL_SETTER") {
        Ok(val) => val,
        Err(_) => String::from("https://api-mn.boulderhalle.app/public-profile/")
    }; 

    let mut fetched = Vec::new();
    for setter_api_id in setters {
        let setterobj: SetterObject = reqwest::Client::new()
            .get(url_setter.clone())
            .send()
            .await?
            .json()
            .await?;

        let data = SetterData::new(
            setter_api_id,
            setterobj.name,
            setterobj.avatar
        );

        let res = create_setter_user(data);

        fetched.push(res);
    }

    if fetched.len() >= 2 {
        return Ok(SetterReturnValue::Two(fetched[0], fetched[1]));
    } else {
        return Ok(SetterReturnValue::One(fetched[0]));
    }
}

pub enum SetterReturnValue {
    One(i32),
    Two(i32, i32)
}