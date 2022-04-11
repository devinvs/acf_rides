use std::thread;

pub fn start() {
    thread::spawn(move || {
        loop {}
    });
}
