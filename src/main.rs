use rides::{db, webserver, rides_finder};
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    db::create_database();
    rides_finder::start();
    webserver::start().await
}
