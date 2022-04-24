use std::thread;

/// Start a new background thread which has a few different functions:
/// 1. Find unassigned riders and assign them to available drivers
/// 2. Find old events and delete them from the database
pub fn start() {
    thread::spawn(move || {
        loop {}
    });
}
