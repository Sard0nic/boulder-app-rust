use crate::db::establish_db_connection;
use crate::models::{AppUser, NewAppUser};
use crate::structs::SetterData;
use chrono::{Local, NaiveDate};
use diesel::prelude::*;

pub fn create_setter_user(user: SetterData) -> i32 {
    use crate::schema::app_users::dsl::*;

    let mut conn = establish_db_connection();

    let new_user = NewAppUser {
        api_id: &user.id,
        username: &user.username,
        password: "Test",
        email: None,
        firstname: None,
        lastname: None,
        setter: true,
        admin: false,
        avatar: &user.avatar,
        created_at: Local::now().date(),
        updated_at: Local::now(),
    };

    let result: AppUser = diesel::insert_into(app_users)
        .values(&new_user)
        .execute(&mut conn)
        .get_results(&mut conn)
        .expect("Unable to create User");

    return user.id;
}

pub fn get_setter_by(api_id_vec: Vec<String>) -> Option<i32> {
    use crate::schema::app_users::dsl::*;

    let mut conn = establish_db_connection();
    let id_str = api_id_vec[0];

    let user: Option<Vec<AppUser>> = match app_users
        .filter(api_id.eq(&id_str))
        .limit(1)
        .load::<AppUser>(&mut conn) {
            Ok(user) => Some(user),
            Err(_) => None
        };

    let res = match user {
        Some(user) => Some(user.id),
        None => None
    };

    return res;
}