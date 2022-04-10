use rides::db::create_database;
use rides::webserver::start_webserver;
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    create_database();
    start_webserver().await
}
