use std::sync::mpsc::Receiver;

use crate::db;

/// Start a new background thread which has a few different functions:
/// 1. Find unassigned riders and assign them to available drivers
/// 2. Find old events and delete them from the database
/// 3. Wait for updates
pub fn start(rx: Receiver<()>) {
    std::thread::spawn(move || {

        loop {
            {
                let conn = db::connect();
                db::delete_old_events(&conn).unwrap();
                db::match_rides(&conn).unwrap();
            }
            rx.recv().unwrap();
        }
    });
}
