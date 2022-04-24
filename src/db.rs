use chrono::NaiveDateTime;
use sqlite::{Connection, Value, State};
use uuid::Uuid;
use log::info;

use std::path::Path;
use std::error::Error;

use crate::models::{User, Campus, Event, Vehicle, Driver};

/// Path for the sqlite database
const DB_PATH: &'static str = "rides.db";

/// Check if the database exists.
/// If not, create the database and run the init script
/// to create the database schema
pub fn create_database() {
    info!("Checking for Database");
    let path = Path::new(DB_PATH);

    if !path.exists() {
        info!("Database not found, creating...");
        let conn = sqlite::open(DB_PATH).unwrap();

        conn.execute(include_str!("./sql/init_database.sql")).unwrap();
    }
}

/// Create a connection to the database
pub fn connect() -> Connection {
    sqlite::open(DB_PATH).unwrap()
}

// Funcions for interacting with Users

/// Create a new user
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

/// Search for a user by their email
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

/// Get a user by their id
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

/// Get a list of all drivers for a given event
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

/// Get a list of all passengers of a driver for a single event
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

/// Create a driver for an event
pub fn create_driver(
    conn: &Connection,
    user_id: Uuid,
    event_id: Uuid,
    vehicle_id: Uuid,
    seats: usize
) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare(
        include_str!("./sql/create_driver.sql")
    )?;

    stmt.bind(1, user_id.to_string().as_str())?;
    stmt.bind(2, event_id.to_string().as_str())?;
    stmt.bind(3, vehicle_id.to_string().as_str())?;
    stmt.bind(4, seats as i64)?;

    loop {
        let state = stmt.next()?;
        if state==State::Done { break; }
    }

    Ok(())
}

/// Create a ride for a given event
pub fn create_ride(
    conn: &Connection,
    user_id: Uuid,
    event_id: Uuid,
    pickup_location: String
) -> Result<(), Box<dyn Error>> {
    info!("Create Ride");
    let mut stmt = conn.prepare(
        include_str!("./sql/create_ride.sql")
    )?;

    stmt.bind(1, user_id.to_string().as_str())?;
    stmt.bind(2, event_id.to_string().as_str())?;
    stmt.bind(3, pickup_location.as_str())?;

    loop {
        let state = stmt.next()?;
        if state==State::Done { break; }
    }

    Ok(())
}

// Event Functions

/// Create a new event
pub fn create_event(
    conn: &Connection,
    name: String,
    time: NaiveDateTime,
    address1: String,
    address2: String,
    city: String,
    state: String,
    zipcode: String,
    owner_id: Uuid
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
    stmt.bind(9, owner_id.to_string().as_str())?;

    loop {
        let state = stmt.next()?;
        if state==State::Done { break; }
    }

    Ok(())
}

/// Get a list of upcoming events
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

/// Create a new vehicle for a driver
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

/// Get all vehicles for a driver
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
        vehicles.push(row.into());
    }

    Ok(vehicles)
}

/// Get vehicle information from its id
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

/// Get all events that a user is driving
pub fn get_driver_events(conn: &Connection, driver_id: Uuid) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut cursor = conn.prepare(
        include_str!("./sql/get_driver_events.sql")
    )?.into_cursor();
    cursor.bind(&[Value::String(driver_id.to_string())])?;

    let mut events = vec![];

    while let Some(row) = cursor.next()? {
        events.push(row.into());
    }

    Ok(events)
}

/// Get all events a user is getting a ride
pub fn get_rider_events(conn: &Connection, rider_id: Uuid) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut cursor = conn.prepare(
        include_str!("./sql/get_rider_events.sql")
    )?.into_cursor();
    cursor.bind(&[Value::String(rider_id.to_string())])?;

    let mut events = vec![];

    while let Some(row) = cursor.next()? {
        events.push(row.into());
    }

    Ok(events)
}

/// Delete a user from an event, whether they are a rider or a driver.
/// Does not delete events for everyone, only removes a user from it
pub fn delete_user_event(conn: &Connection, user_id: Uuid, event_id: Uuid) -> Result<(), Box<dyn Error>> {
    info!("Removing user from event");

    let mut remove_rides = conn.prepare(
        include_str!("./sql/delete_user_rides.sql")
    )?;

    let mut remove_drives = conn.prepare(
        include_str!("./sql/delete_user_rides_drives.sql")
    )?;

    let mut remove_drivers = conn.prepare(
        include_str!("./sql/delete_user_drives.sql")
    )?;

    let user_id = user_id.to_string();
    let event_id = event_id.to_string();

    remove_rides.bind(1, user_id.as_str())?;
    remove_rides.bind(2, event_id.as_str())?;

    remove_drives.bind(1, user_id.as_str())?;
    remove_drives.bind(2, event_id.as_str())?;

    remove_drivers.bind(1, user_id.as_str())?;
    remove_drivers.bind(2, event_id.as_str())?;

    // Begin Transaction
    conn.execute("BEGIN;")?;

    // Remove Rides
    loop {
        let state = remove_rides.next()?;
        if state==State::Done { break; }
    }

    // Remove Drives
    loop {
        let state = remove_drives.next()?;
        if state==State::Done { break; }
    }

    // Remove drivers
    loop {
        let state = remove_drivers.next()?;
        if state == State::Done { break; }
    }

    // End Transaction
    conn.execute("COMMIT;")?;

    Ok(())
}
