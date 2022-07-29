use std::time::Duration;

use crate::db;

/// Start a new background thread which has a few different functions:
/// 1. Find unassigned riders and assign them to available drivers
/// 2. Find old events and delete them from the database
/// 3. Sleep
pub fn start() {
    std::thread::spawn(|| {
        loop {
            let conn = db::connect();
            db::delete_old_events(&conn).unwrap();
            db::match_rides(&conn).unwrap();
            std::thread::sleep(Duration::from_secs(10));
        }
    });
}
