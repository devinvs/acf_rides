use log::info;
use actix_web::{App, HttpServer, Responder, get, HttpResponse, post, web};
use actix_web::cookie::Key;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use crate::db;

macro_rules! auth {
    ($s:ident) => {
        let logged_in = $s.get::<bool>("logged_in")?;
        if logged_in.is_none() || !logged_in.unwrap() {
            return Ok(
                HttpResponse::MovedPermanently()
                .append_header(("Location", "/login"))
                .finish()
            )
        }
    };
}

fn read_file(path: &str) -> HttpResponse {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    HttpResponse::Ok().body(buf)
}


#[get("/")]
async fn get_root(s: Session) -> Result<impl Responder, Box<dyn Error>> {
    auth!(s);
    Ok(read_file("./public/drive_or_ride.html"))
}

#[get("/login")]
async fn get_login() -> impl Responder {
    read_file("./public/login.html")
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    password: String
}

#[post("/login")]
async fn post_login(s: Session, form: web::Form<FormData>) -> impl Responder {
    let conn = db::connect();
    let user = db::get_user_by_email(&conn, form.email.clone()).unwrap();

    if let Some(u) = user {
        let verify = bcrypt::verify(form.password.clone(), u.password.as_str());
        if verify.is_ok() && verify.unwrap() {
            s.insert("logged_in", true).unwrap();

            return HttpResponse::MovedPermanently()
                .append_header(("Location", "/"))
                .finish();
        }
    }

    read_file("./public/login.html")
}

#[get("/events")]
async fn get_events(s: Session) -> Result<impl Responder, Box<dyn Error>> {
    auth!(s);
    Ok(read_file("./public/events.html"))
}

#[get("/vehicles")]
async fn get_vehicles(s: Session) -> Result<impl Responder, Box<dyn Error>> {
    auth!(s);
    Ok(read_file("./public/vehicles.html"))
}

#[get("/signup")]
async fn get_signup() -> impl Responder {
    read_file("./public/signup.html")
}

#[get("/manage_events")]
async fn get_manage_events(s: Session) -> Result<impl Responder, Box<dyn Error>> {
    auth!(s);
    Ok(read_file("./public/manage_events.html"))
}

#[get("/reset")]
async fn get_reset_password() -> impl Responder {
    read_file("./public/reset_password.html")
}

pub async fn start() -> std::io::Result<()> {
    info!("Starting Webserver");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
                .cookie_secure(true)
                .build()
            )
            .service(get_root)
            .service(get_login)
            .service(post_login)
            .service(get_events)
            .service(get_vehicles)
            .service(get_signup)
            .service(get_manage_events)
            .service(get_reset_password)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
