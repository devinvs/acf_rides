use sqlite::Value;
use uuid::Uuid;
use chrono::NaiveDateTime;

pub enum Campus {
    RIT,
    UofR
}

impl From<&str> for Campus {
    fn from(s: &str) -> Self {
        match s {
            "RIT" => Campus::RIT,
            _ => Campus::UofR
        }
    }
}

impl Into<&'static str> for Campus {
    fn into(self) -> &'static str {
        match self {
            Campus::RIT => "RIT",
            Campus::UofR => "UR"
        }
    }
}

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub fullname: String,
    pub password: String,
    pub number: String,
    pub campus: Campus
}

impl From<&[Value]> for User {
    fn from(row: &[Value]) -> Self {
        let id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
        let email = row[1].as_string().unwrap().to_string();
        let fullname = row[2].as_string().unwrap().to_string();
        let password = row[3].as_string().unwrap().to_string();
        let number = row[4].as_string().unwrap().to_string();
        let campus = Campus::from(row[5].as_string().unwrap());

        User {
            id,
            email,
            fullname,
            password,
            number,
            campus
        }
    }
}

pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub creator_id: Uuid
}

impl From<&[Value]> for Event {
    fn from(row: &[Value]) -> Self {
        let id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
        let name = row[1].as_string().unwrap().to_string();
        let time = NaiveDateTime::from_timestamp(
            row[2].as_integer().unwrap(),
            0
        );
        let address1 = row[3].as_string().unwrap().to_string();
        let address2 = row[4].as_string().unwrap().to_string();
        let city = row[5].as_string().unwrap().to_string();
        let state = row[6].as_string().unwrap().to_string();
        let zipcode = row[7].as_string().unwrap().to_string();
        let creator_id = Uuid::parse_str(row[8].as_string().unwrap()).unwrap();

        Event {
            id,
            name,
            time,
            address1,
            address2,
            city,
            state,
            zipcode,
            creator_id
        }
    }
}

pub struct Vehicle {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub color: String,
    pub make: String,
    pub model: String,
}

impl From<&[Value]> for Vehicle {
    fn from(row: &[Value]) -> Self {
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
}

pub struct Driver {
    pub event_id: Uuid,
    pub driver_id: Uuid,
    pub seats: i64,
    pub vehicle_id: Uuid,
}

impl From<&[Value]> for Driver {
    fn from(row: &[Value]) -> Self {
        let event_id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
        let driver_id = Uuid::parse_str(row[1].as_string().unwrap()).unwrap();
        let seats = row[2].as_integer().unwrap();
        let vehicle_id = Uuid::parse_str(row[3].as_string().unwrap()).unwrap();

        Driver {
            event_id,
            driver_id,
            seats,
            vehicle_id
        }
    }
}

pub struct Ride {
    pub rider_id: Uuid,
    pub driver_id: Option<Uuid>,
    pub event_id: Uuid,
    pub pickup_location: String
}
