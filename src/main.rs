use rides::{db, webserver, rides_finder};
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    db::create_database();
    rides_finder::start();
    webserver::start().await
}
