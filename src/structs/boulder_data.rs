use chrono::NaiveDateTime;

use crate::{scraping::BoulderObject};

pub struct BoulderData {
    pub gradenum: i32,
    pub api_id: String,
    pub grade: i32,
    pub nr: i32,
    pub removed: bool,
    pub section: String,
    pub setter_id: i32,
    pub setter_two_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

impl BoulderData {
    pub fn new(data: BoulderObject, setter_id: i32, setter_two_id: Option<i32>) -> BoulderData {
        let removed = boolean_from_number(data.content.removed);
        let grade = grade_to_i32(data.content.grade);
        let created_at = Some(usize_to_date(data.content.setDate));

        // Generate gradenum
        let grade_num = calc_id(grade, data.content.gradeNr).unwrap();

        BoulderData {
            gradenum: grade_num,
            api_id: data.id,
            grade,
            nr: data.content.gradeNr,
            removed,
            section: data.content.sector,
            setter_id,
            setter_two_id,
            created_at,
        }
    }
}

pub fn calc_id(grade: i32, nr: i32) -> Result<i32, String> {
    let grade_string: String = grade.to_string();
    let nr_string: String = nr.to_string();

    let grade_long = if grade_string.chars().count() >= 2 {
        grade_string
    } else {
        format!("{}{}", "0", grade_string)
    };

    let nr_long = if nr_string.chars().count() >= 2 {
        nr_string
    } else {
        format!("{}{}", "0", nr_string)
    };

    let id_string = format!("{}{}", grade_long, nr_long);

    let id_num: i32 = match id_string.parse::<i32>() {
        Ok(id) => id,
        Err(e) => return Err(String::from(format!("Invalid Value {}", e)))
    };

    return Ok(id_num);
}

fn boolean_from_number(num: i32) -> bool {
    let res = match num {
        0 => false,
        1 => true,
        _ => false
    };

    return res
}

fn grade_to_i32(grade_str: String) -> i32 {
    let mut res = 0;
    if grade_str == String::from("yellow") {
        res = 1
    } else if grade_str == String::from("green") {
        res = 2
    } else if grade_str == String::from("orange") {
        res = 3
    } else if grade_str == String::from("blue") {
        res = 4
    } else if grade_str == String::from("red") {
        res = 5
    } else if grade_str == String::from("white") {
        res = 6
    } else if grade_str == String::from("black") {
        res = 7
    }

    return res;
}

fn usize_to_date(date: i64) -> NaiveDateTime {
    return NaiveDateTime::from_timestamp(date, 0);
}