use rides::{db, webserver, worker};
use actix_web;

use std::sync::mpsc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().unwrap_or_default();

    // Initialize Logging
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Create database if it doesn't exist
    db::create_database();

    // Create comms for api -> worker
    let (tx, rx) = mpsc::channel::<()>();

    // Start the background thread
    worker::start(rx);

    // Start the webserver
    webserver::start(tx).await
}
