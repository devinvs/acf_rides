use chrono::NaiveDateTime;
use sqlite::{Connection, Value, State};
use uuid::Uuid;
use std::{path::Path, error::Error};
use log::info;

use crate::models::{User, Campus, Event, Vehicle};

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

// Funcions for interacting with Users
pub fn create_user(conn: &Connection, email: String, fullname: String, password: String, number: String, campus: Campus) -> Result<(), Box<dyn Error>> {
    info!("Creating New User: {email}");
    let id = Uuid::new_v4().to_string();
    let hash = bcrypt::hash(password, 7)?;

    let mut stmt = conn.prepare(include_str!("./sql/create_user.sql"))?;

    stmt.bind(1, id.as_str())?;
    stmt.bind(2, email.as_str())?;
    stmt.bind(3, fullname.as_str())?;
    stmt.bind(4, hash.as_str())?;
    stmt.bind(5, number.as_str())?;
    stmt.bind(6, campus.to_string().as_str())?;

    loop {
        let state = stmt.next()?;
        if state == State::Done { break; }
    }

    Ok(())
}

fn row_to_user(row: &[Value]) -> User {
    let id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
    let email = row[1].as_string().unwrap().to_string();
    let fullname = row[2].as_string().unwrap().to_string();
    let password = row[3].as_string().unwrap().to_string();
    let number = row[4].as_string().unwrap().to_string();
    let campus = row[5].as_string().unwrap();

    let campus = if campus=="RIT" {Campus::RIT} else {Campus::UofR};

    User {
        id,
        email,
        fullname,
        password,
        number,
        campus
    }
}

pub fn get_user_by_email(conn: &Connection, email: String) -> Result<Option<User>, Box<dyn Error>> {
    info!("Finding User with email: {email}");
    let mut cursor = conn.prepare(include_str!("./sql/get_user_by_email.sql"))?.into_cursor();

    cursor.bind(&[Value::String(email)])?;

    let row = cursor.next()?;
    if row.is_none() { return Ok(None); }
    let row = row.unwrap();

    Ok(Some(row_to_user(row)))
}

pub fn get_user(conn: &Connection, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
    info!("Finding User with id: {id}");

    let mut cursor = conn.prepare(include_str!("./sql/get_user.sql"))?.into_cursor();

    cursor.bind(&[Value::String(id.to_string())])?;

    let row = cursor.next()?;
    if row.is_none() { return Ok(None) };
    let row = row.unwrap();

    Ok(Some(row_to_user(row)))
}


// Event Functions
pub fn create_event(conn: &Connection, name: String, time: NaiveDateTime, address1: String, address2: String, city: String, state: String, zipcode: String) -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4().to_string();
    let mut stmt = conn.prepare(include_str!("./sql/create_event.sql"))?;

    stmt.bind(1, id.as_str())?;
    stmt.bind(2, name.as_str())?;
    stmt.bind(3, time.timestamp())?;
    stmt.bind(4, address1.as_str())?;
    stmt.bind(5, address2.as_str())?;
    stmt.bind(6, city.as_str())?;
    stmt.bind(7, state.as_str())?;
    stmt.bind(8, zipcode.as_str())?;

    loop {
        let state = stmt.next()?;
        if state==State::Done { break; }
    }

    Ok(())
}

fn row_to_event(row: &[Value]) -> Event {
    let id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
    let name = row[1].as_string().unwrap().to_string();
    let time = NaiveDateTime::from_timestamp(
        row[2].as_integer().unwrap(),
        0
    );
    let address1 = row[3].as_string().unwrap().to_string();
    let address2 = row[4].as_string().unwrap().to_string();
    let city = row[6].as_string().unwrap().to_string();
    let state = row[6].as_string().unwrap().to_string();
    let zipcode = row[7].as_string().unwrap().to_string();

    Event {
        id,
        name,
        time,
        address1,
        address2,
        city,
        state,
        zipcode
    }
}

// Vehicles functions
pub fn create_vehicle(conn: &Connection, user_id: Uuid, color: String, make: String, model: String) -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4().to_string();

    let mut stmt = conn.prepare(include_str!("./sql/create_vehicle.sql"))?;

    stmt.bind(1, id.as_str())?;
    stmt.bind(2, user_id.to_string().as_str())?;
    stmt.bind(3, color.as_str())?;
    stmt.bind(4, make.as_str())?;
    stmt.bind(5, model.as_str())?;

    loop {
        let state = stmt.next()?;
        if state==State::Done { break; }
    }

    Ok(())
}

fn row_to_vehicle(row: &[Value]) -> Vehicle {
    let id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
    let owner_id = Uuid::parse_str(row[1].as_string().unwrap()).unwrap();
    let color = row[2].as_string().unwrap().to_string();
    let make = row[3].as_string().unwrap().to_string();
    let model = row[4].as_string().unwrap().to_string();

    Vehicle {
        id,
        owner_id,
        color,
        make,
        model
    }
}

pub fn get_vehicle(conn: &Connection, id: Uuid) -> Result<Option<Vehicle>, Box<dyn Error>> {

    let mut cursor = conn.prepare(include_str!("./sql/get_vehicle.sql"))?.into_cursor();

    cursor.bind(&[Value::String(id.to_string())])?;

    let row = cursor.next()?;
    if row.is_none() { return Ok(None) };
    let row = row.unwrap();

    Ok(Some(row_to_vehicle(row)))
}
