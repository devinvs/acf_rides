use chrono::NaiveDateTime;
use sqlite::{Connection, Value, State};
use uuid::Uuid;
use log::info;

use std::path::Path;
use std::error::Error;

use crate::models::{User, Campus, Event, Vehicle, Driver};

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
pub fn create_user(
    conn: &Connection,
    email: String,
    fullname: String,
    password: String,
    number: String,
    campus: Campus
) -> Result<(), Box<dyn Error>> {
    info!("Creating New User: {email}");
    let id = Uuid::new_v4().to_string();
    let hash = bcrypt::hash(password, 7)?;

    let mut stmt = conn.prepare(include_str!("./sql/create_user.sql"))?;

    let campus: &'static str = campus.into();

    stmt.bind(1, id.as_str())?;
    stmt.bind(2, email.as_str())?;
    stmt.bind(3, fullname.as_str())?;
    stmt.bind(4, hash.as_str())?;
    stmt.bind(5, number.as_str())?;
    stmt.bind(6, campus)?;

    loop {
        let state = stmt.next()?;
        if state == State::Done { break; }
    }

    Ok(())
}

pub fn get_user_by_email(
    conn: &Connection,
    email: String
) -> Result<Option<User>, Box<dyn Error>> {
    info!("Finding User with email: {email}");
    let mut cursor = conn.prepare(
        include_str!("./sql/get_user_by_email.sql")
    )?.into_cursor();

    cursor.bind(&[Value::String(email)])?;
    let row = cursor.next()?;

    if row.is_none() { return Ok(None); }
    let row = row.unwrap();

    Ok(Some(row.into()))
}

pub fn get_user(
    conn: &Connection,
    id: Uuid
) -> Result<Option<User>, Box<dyn Error>> {
    info!("Finding User with id: {id}");

    let mut cursor = conn.prepare(
        include_str!("./sql/get_user.sql")
    )?.into_cursor();

    cursor.bind(&[Value::String(id.to_string())])?;

    let row = cursor.next()?;
    if row.is_none() { return Ok(None) };
    let row = row.unwrap();

    Ok(Some(row.into()))
}

pub fn get_riders(
    conn: &Connection,
    event_id: Uuid,
    driver_id: Uuid
) -> Result<Vec<User>, Box<dyn Error>> {
    info!("Getting riders for driver");
    let mut cursor = conn.prepare(
        include_str!("./sql/get_riders.sql")
    )?.into_cursor();

    cursor.bind(&[
        Value::String(event_id.to_string()),
        Value::String(driver_id.to_string())
    ])?;

    let mut users = Vec::new();
    while let Some(row) = cursor.next()? {
        users.push(row.into());
    }

    Ok(users)
}

pub fn get_available_drivers(
    conn: &Connection,
    event_id: Uuid,
    campus: Campus,
) -> Result<Vec<Driver>, Box<dyn Error>> {
    info!("Getting available drivers for event");
    let mut cursor = conn.prepare(
        include_str!("./sql/get_available_drivers.sql")
    )?.into_cursor();

    let campus: &'static str = campus.into();

    cursor.bind(&[
        Value::String(event_id.to_string()),
        Value::String(campus.into())
    ])?;

    let mut drivers = Vec::new();

    while let Some(row) = cursor.next()? {
        drivers.push(row.into());
    }

    Ok(drivers)
}

pub fn get_driver_passengers(
    conn: &Connection,
    event_id: Uuid,
    driver_id: Uuid
) -> Result<Vec<(User, String)>, Box<dyn Error>> {
    info!("Get passengers for a driver");
    let mut cursor = conn.prepare(
        include_str!("./sql/get_driver_passengers.sql")
    )?.into_cursor();

    cursor.bind(&[
        Value::String(event_id.to_string()),
        Value::String(driver_id.to_string())
    ])?;

    let mut passengers = Vec::new();

    while let Some(row) = cursor.next()? {
        passengers.push((row.into(), row[6].as_string().unwrap().to_string()));
    }

    Ok(passengers)
}

// Event Functions
pub fn create_event(
    conn: &Connection,
    name: String,
    time: NaiveDateTime,
    address1: String,
    address2: String,
    city: String,
    state: String,
    zipcode: String
) -> Result<(), Box<dyn Error>> {
    info!("Create event: {name}");
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

pub fn get_events(conn: &Connection) -> Result<Vec<Event>, Box<dyn Error>> {
    info!("Get events");
    let mut cursor = conn.prepare(
        include_str!("./sql/get_events.sql")
    )?.into_cursor();

    let mut events = Vec::new(); while let Some(row) = cursor.next()? {
        events.push(row.into());
    }

    Ok(events)
}

// Vehicles functions
pub fn create_vehicle(
    conn: &Connection,
    user_id: Uuid,
    color: String,
    make: String,
    model: String
) -> Result<(), Box<dyn Error>> {
    info!("Create vehicle: {make} {model}");
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

pub fn get_driver_vehicles(
    conn: &Connection,
    driver_id: Uuid
) -> Result<Vec<Vehicle>, Box<dyn Error>> {
    info!("Get driver's vehicles");
    let mut cursor = conn.prepare(
        include_str!("./sql/get_driver_vehicles.sql")
    )?.into_cursor();

    cursor.bind(&[Value::String(driver_id.to_string())])?;
    let mut vehicles = Vec::new();

    while let Some(row) = cursor.next()? {
        println!("{:?}", row);
        vehicles.push(row.into());
    }

    Ok(vehicles)
}

pub fn get_vehicle(
    conn: &Connection,
    id: Uuid
) -> Result<Option<Vehicle>, Box<dyn Error>> {
    info!("Get vehicle: {id}");

    let mut cursor = conn.prepare(
        include_str!("./sql/get_vehicle.sql")
    )?.into_cursor();

    cursor.bind(&[Value::String(id.to_string())])?;

    let row = cursor.next()?;
    if row.is_none() { return Ok(None) };
    let row = row.unwrap();

    Ok(Some(row.into()))
}

