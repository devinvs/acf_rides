use sqlite::Value;
use uuid::Uuid;
use chrono::NaiveDateTime;

/// Available campus locations
/// A driver can only give rides for people on their campus
pub enum Campus {
    /// Rochester Institute of Technology
    RIT,
    /// University of Rochester
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

impl Into<String> for Campus {
    fn into(self) -> String {
        match self {
            Campus::RIT => String::from("RIT"),
            Campus::UofR => String::from("UR")
        }
    }
}

/// A User of the App
/// Can act as a driver or a rider
pub struct User {
    /// Unique User ID
    pub id: Uuid,
    pub email: String,
    pub fullname: String,
    /// Hashed using BCrypt
    pub password: String,
    /// Phone Number
    pub number: String,
    /// Default campus for a user
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

/// A single event that people need rides from/can provide rides to
/// Events that have passed will be deleted by a background thread
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub time: NaiveDateTime,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    /// ID of the user who created and can delete this event
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

/// Event Metaobject, containing all information that a driver/rider would need
pub struct EventData {
    pub event: Event,
    /// List of tuples of riders and their pickup location
    pub riders: Option<Vec<(User, String)>>,
    /// Tuple of Drivers and their vehicle
    pub driver: Option<(User, Vehicle)>,
    /// Is Driver
    pub is_driver: bool
}

/// Information about a driver's vehicle
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

/// A driver for a single event.
pub struct Driver {
    /// The event id that the driver will drive for
    pub event_id: Uuid,
    /// The user id of the driver
    pub driver_id: Uuid,
    /// Number of total seats
    pub seats: i64,
    /// Vehicle id of the vehicle the driver will be driving
    pub vehicle_id: Uuid,
    /// Campus the driver will be driving from
    pub campus: Campus
}

impl From<&[Value]> for Driver {
    fn from(row: &[Value]) -> Self {
        let event_id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
        let driver_id = Uuid::parse_str(row[1].as_string().unwrap()).unwrap();
        let seats = row[2].as_integer().unwrap();
        let vehicle_id = Uuid::parse_str(row[3].as_string().unwrap()).unwrap();
        let campus: Campus = row[4].as_string().unwrap().into();

        Driver {
            event_id,
            driver_id,
            seats,
            vehicle_id,
            campus
        }
    }
}

/// A single ride for a single event
pub struct Ride {
    /// The user id of the rider
    pub rider_id: Uuid,
    /// The user id of the driver
    pub driver_id: Option<Uuid>,
    /// The id of the event
    pub event_id: Uuid,
    /// The campus to be picked up from
    pub campus: Campus,
    /// The location the rider wants to be picked up
    pub pickup_location: String
}

impl From<&[Value]> for Ride {
    fn from(row: &[Value]) -> Self {
        let rider_id = Uuid::parse_str(row[0].as_string().unwrap()).unwrap();
        let driver_id = if row[1] == Value::Null {
            None
        } else {
            Some(Uuid::parse_str(row[1].as_string().unwrap()).unwrap())
        };
        let event_id = Uuid::parse_str(row[2].as_string().unwrap()).unwrap();
        let campus: Campus = row[3].as_string().unwrap().into();
        let pickup_location = row[4].as_string().unwrap().to_string();

        Ride {
            rider_id,
            driver_id,
            event_id,
            campus,
            pickup_location
        }
    }
}
