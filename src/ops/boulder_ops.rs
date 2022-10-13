use crate::db::establish_db_connection;
use crate::models::{Boulder, NewBoulder};
use crate::structs::BoulderData;
use chrono::{Datelike, Local, NaiveDate};
use diesel::prelude::*;

pub fn create_boulder(boulder: BoulderData) {
    use crate::schema::boulders::dsl::*;

    let mut conn = establish_db_connection();

    let new_boulder = NewBoulder {
        gradenum: boulder.gradenum,
        api_id: &boulder.api_id,
        grade: boulder.grade,
        nr: boulder.nr,
        removed: boulder.removed,
        section: &boulder.section,
        setter_id: boulder.setter_id,
        setter_two_id: boulder.setter_two_id,
        created_at: match boulder.created_at {
            Some(date) => &date,
            None => &get_naivedate_now(),
        },
        updated_at: Local::now(),
    };

    diesel::insert_into(boulders)
        .values(&new_boulder)
        .execute(&mut conn)
        .expect("Error saving new Boulder");
}

// pub fn update_boulder(boulder: Boulder) {
//     use crate::schema::boulders::dsl::*;

//     let mut conn = establish_db_connection();

//     let db_boulder = Boulder {
//         id: boulder.id,
//         api_id: boulder.api_id,
//         grade: boulder.grade,
//         nr: boulder.nr,
//         removed: boulder.removed,
//     };

//     diesel::update(boulders.find(boulder.id))
//         .set(&db_boulder)
//         .execute(&mut conn)
//         .expect("Error updating Boulder");
// }

// pub fn get_all_boulders() -> Vec<Boulder> {
//     use crate::schema::boulders::dsl::*;

//     let mut conn = establish_db_connection();

//     let result = boulders
//         .filter(removed.eq(false))
//         .load::<Boulder>(&mut conn)
//         .expect("Error loading boulders");

//     return result;
// }

fn get_naivedate_now() -> NaiveDate {
    let date = Local::now();
    let naive = NaiveDate::from_ymd(date.year(), date.month(), date.day());

    return naive;
}
