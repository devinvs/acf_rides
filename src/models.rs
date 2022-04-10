use uuid::Uuid;
use chrono::NaiveDateTime;

pub enum Campus {
    RIT,
    UofR
}

impl Campus {
    pub fn to_string(self) -> String {
        match self {
            Campus::RIT => "RIT".into(),
            Campus::UofR => "UR".into()
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

pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zipcode: String
}

pub struct Vehicle {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub color: String,
    pub make: String,
    pub model: String,
}

pub struct Driver {
    pub event_id: Uuid,
    pub driver_id: Uuid,
    pub seats: usize,
    pub vehicle_id: Uuid,
}

pub struct Ride {
    pub rider_id: Uuid,
    pub driver_id: Option<Uuid>,
    pub event_id: Uuid,
    pub pickup_location: String
}
