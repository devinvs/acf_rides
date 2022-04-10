use log::info;
use actix_web::{App, HttpServer, Responder, get};
use crate::db;

#[get("/")]
async fn get_root() -> impl Responder {
    "Root"
}

#[get("/login")]
async fn get_login() -> impl Responder {
    "Login"
}

#[get("/events")]
async fn get_events() -> impl Responder {
    "Events"
}

#[get("/vehicles")]
async fn get_vehicles() -> impl Responder {
    "Vehicles"
}

pub async fn start_webserver() -> std::io::Result<()> {
    info!("Starting Webserver");

    HttpServer::new(|| {
        App::new()
            .service(get_root)
            .service(get_login)
            .service(get_events)
            .service(get_vehicles)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
