use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please specify 'DATABASE_URL' in '.env'");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to: {}", &database_url))
}