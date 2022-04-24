use rides::{db, webserver, rides_finder};
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().unwrap();

    // Initialize Logging
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Create database if it doesn't exist
    db::create_database();

    // Start the background thread
    rides_finder::start();

    // Start the webserver
    webserver::start().await
}
