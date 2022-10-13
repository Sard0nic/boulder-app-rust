use chrono::NaiveDate;

// Boulders Schema
use crate::schema::boulders;

#[derive(Insertable)]
#[table_name = "boulders"]
pub struct NewBoulder<'a> {
    pub gradenum: i32,
    pub api_id: &'a str,
    pub grade: i32,
    pub nr: i32,
    pub removed: bool,
    pub section: &'a str,
    pub setter_id: i32,
    pub setter_two_id: i32,
    pub created_at: &'a NaiveDate,
    pub updated_at: &'a NaiveDate,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Boulder {
    pub id: i32,
    pub gradenum: i32,
    pub api_id: String,
    pub grade: i32,
    pub nr: i32,
    pub removed: bool,
    pub section: String,
    pub setter_id: i32,
    pub setter_two_id: i32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// Users Schema
use crate::schema::app_users;

#[derive(Insertable)]
#[table_name = "app_users"]
pub struct NewAppUser<'a> {
    pub username: &'a str,
    pub api_id: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub avatar: &'a str,
    pub setter: bool,
    pub admin: bool,
    pub created_at: &'a NaiveDate,
    pub updated_at: &'a NaiveDate,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct AppUser {
    pub id: i32,
    pub api_id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub avatar: String,
    pub setter: bool,
    pub admin: bool,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// Attempts Schema
use crate::schema::attempts;

#[derive(Insertable)]
#[table_name = "attempts"]
pub struct NewAttempt<'a> {
    pub user_id: i32,
    pub boulder_id: i32,
    pub created_at: &'a NaiveDate,
    pub updated_at: &'a NaiveDate,
    pub att_count: i32,
    pub topped: bool,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Attempt {
    pub id: i32,
    pub user_id: i32,
    pub boulder_id: i32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub att_count: i32,
    pub topped: bool,
}
