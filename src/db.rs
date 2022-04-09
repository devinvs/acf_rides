use sqlite::Connection;
use std::path::Path;
use log::info;

const DB_PATH: &'static str = "rides.db";

pub fn create_database() {
    info!("Checking for Database");
    let path = Path::new(DB_PATH);

    if !path.exists() {
        info!("Database not found, creating...");
        let conn = sqlite::open(DB_PATH).unwrap();

        conn.execute(include_str!("./sql/init_database.sql")).unwrap();
    }
}

pub fn connect() -> Connection {
    sqlite::open(DB_PATH).unwrap()
}
