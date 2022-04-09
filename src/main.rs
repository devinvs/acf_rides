use log::info;
use rides::db::create_database;

fn main() {
    env_logger::init();
    create_database();
    start_webserver();
}

fn start_webserver() {
    info!("Starting Webserver");
}
