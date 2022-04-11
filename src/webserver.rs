use chrono::NaiveDateTime;
use log::info;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, Responder, get, HttpResponse, post, web};
use actix_web::cookie::Key;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use serde::Deserialize;
use uuid::Uuid;
use crate::db;
use crate::models::{Campus, Event, Vehicle};
use askama::Template;
use std::fs::File;
use std::io::Read;

// Templates
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: String
}

#[derive(Template)]
#[template(path = "summary.html")]
struct SummaryTemplate {}

#[derive(Template)]
#[template(path = "events.html")]
struct EventsTemplate {
    events: Vec<Event>,
    href: String
}

#[derive(Template)]
#[template(path = "pickup.html")]
struct PickupTemplate {}

#[derive(Template)]
#[template(path = "manage_events.html")]
struct ManageEventsTemplate {}

#[derive(Template)]
#[template(path = "reset_password.html")]
struct ResetPasswordTemplate {}

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate {
    error: String
}

#[derive(Template)]
#[template(path = "vehicles.html")]
struct VehiclesTemplate {
    vehicles: Vec<Vehicle>
}

#[derive(Template)]
#[template(path = "seats.html")]
struct SeatsTemplate {}

macro_rules! auth {
    ($s:ident) => {
        let logged_in = $s.get::<bool>("logged_in").unwrap();
        if logged_in.is_none() || !logged_in.unwrap() {
            return
                HttpResponse::SeeOther()
                .append_header(("Location", "/login"))
                .finish();
        }
    };
}

#[get("/css")]
async fn get_css() -> impl Responder {
    let mut f = File::open("./public/style.css").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    HttpResponse::Ok()
        .content_type("text/css")
        .body(buf)
}

#[get("/")]
async fn get_root(s: Session) -> impl Responder {
    auth!(s);

    HttpResponse::Ok().body(SummaryTemplate {}.render().unwrap())
}

#[get("/login")]
async fn get_login() -> impl Responder {
    LoginTemplate {
        error: "".into()
    }
}

#[derive(Deserialize)]
struct LoginFormData {
    email: String,
    password: String
}

#[post("/login")]
async fn post_login(s: Session, form: web::Form<LoginFormData>) -> impl Responder {
    let conn = db::connect();
    let user = db::get_user_by_email(&conn, form.email.clone()).unwrap();

    if let Some(u) = user {
        let verify = bcrypt::verify(form.password.clone(), u.password.as_str());
        if verify.is_ok() && verify.unwrap() {
            s.insert("logged_in", true).unwrap();
            s.insert("user_id", u.id.to_string()).unwrap();

            return HttpResponse::SeeOther()
                .append_header(("Location", "/"))
                .finish();
        }
    }

    HttpResponse::Ok().body(
        LoginTemplate{
            error: "Email/Password Incorrect".into()
        }.render().unwrap()
    )
}

#[derive(Deserialize)]
struct FlowQuery {
    flow: String
}

#[get("/events")]
async fn get_events(s: Session, flow: web::Query<FlowQuery>) -> impl Responder {
    auth!(s);

    let href = match flow.flow.as_str() {
        "drive" => "/vehicles",
        "ride" => "/pickup",
        _ => {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/"))
                .finish()
        }
    }.to_string();

    s.insert("flow", flow.flow.clone()).unwrap();

    let conn = db::connect();
    let events = db::get_events(&conn).unwrap();

    HttpResponse::Ok().body(
        EventsTemplate {
            events,
            href
        }.render().unwrap()
    )
}

#[get("/pickup")]
async fn get_pickup(s: Session, q: web::Query<EventQuery>) -> impl Responder {
    auth!(s);

    s.insert("event_id", q.event_id.clone()).unwrap();

    HttpResponse::Ok()
        .body(PickupTemplate {}.render().unwrap())
}

#[derive(Deserialize)]
struct PickupData {
    pickup: String
}

#[post("/pickup")]
async fn post_pickup(s: Session, form: web::Form<PickupData>) -> impl Responder {
    auth!(s);

    let id: String = s.get("user_id").unwrap().unwrap();
    let id = Uuid::parse_str(id.as_str()).unwrap();

    let event_id: String = s.get("event_id").unwrap().unwrap();
    let event_id = Uuid::parse_str(event_id.as_str()).unwrap();

    let pickup = form.pickup.clone();

    let conn = db::connect();
    db::create_ride(&conn, id, event_id, pickup).unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}

#[derive(Deserialize)]
struct EventQuery {
    event_id: String,
}

#[get("/vehicles")]
async fn get_vehicles(s: Session, q: web::Query<EventQuery>) -> impl Responder {
    auth!(s);

    s.insert("event_id", q.event_id.clone()).unwrap();

    let id: String = s.get("user_id").unwrap().unwrap();
    let id = Uuid::parse_str(id.as_str()).unwrap();

    let conn = db::connect();
    let vehicles = db::get_driver_vehicles(&conn, id).unwrap();

    HttpResponse::Ok().body(
        VehiclesTemplate {
            vehicles
        }.render().unwrap()
    )
}

#[derive(Deserialize)]
struct VehicleFormData {
    make: String,
    model: String,
    color: String
}

#[post("/vehicles")]
async fn post_vehicle(s: Session, form: web::Form<VehicleFormData>) -> impl Responder {
    auth!(s);

    let id: String = s.get("user_id").unwrap().unwrap();
    let id = Uuid::parse_str(id.as_str()).unwrap();

    let conn = db::connect();
    db::create_vehicle(&conn, id, form.color.clone(), form.make.clone(), form.model.clone()).unwrap();
    let vehicles = db::get_driver_vehicles(&conn, id).unwrap();

    HttpResponse::Ok().body(
        VehiclesTemplate {
            vehicles
        }.render().unwrap()
    )
}

#[derive(Deserialize)]
struct VehicleQuery {
    vehicle_id: String
}

#[get("/seats")]
async fn get_seats(s: Session, q: web::Query<VehicleQuery>) -> impl Responder {
    auth!(s);

    s.insert("vehicle_id", q.vehicle_id.clone()).unwrap();

    HttpResponse::Ok()
        .body(SeatsTemplate{}.render().unwrap())
}

#[derive(Deserialize)]
struct SeatsData {
    seats: usize
}

#[post("/seats")]
async fn post_seats(s: Session, form: web::Form<SeatsData>) -> impl Responder {
    auth!(s);

    let id: String = s.get("user_id").unwrap().unwrap();
    let id = Uuid::parse_str(id.as_str()).unwrap();

    let event_id: String = s.get("event_id").unwrap().unwrap();
    let event_id = Uuid::parse_str(event_id.as_str()).unwrap();

    let vehicle_id: String = s.get("vehicle_id").unwrap().unwrap();
    let vehicle_id = Uuid::parse_str(vehicle_id.as_str()).unwrap();

    let conn = db::connect();
    db::create_driver(&conn, id, event_id, vehicle_id, form.seats).unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}

#[get("/signup")]
async fn get_signup() -> impl Responder {
    SignupTemplate {
        error: "".into()
    }
}

#[derive(Deserialize)]
struct SignupFormData {
    name: String,
    email: String,
    password: String,
    confirm_password: String,
    campus: String,
    phone: String
}

#[post("/signup")]
async fn post_signup(s: Session, form: web::Form<SignupFormData>) -> impl Responder {

    if !form.email.ends_with("@rit.edu")
    && !form.email.ends_with("@g.rit.edu")
    && !form.email.ends_with("@u.rochester.edu") {
        return HttpResponse::Ok().body(
            SignupTemplate {
                error: "Must have a .edu email".into()
            }.render().unwrap()
        );
    }

    if form.password != form.confirm_password {
        return HttpResponse::Ok().body(
            SignupTemplate {
                error: "Passwords do not match".into()
            }.render().unwrap()
        );
    }

    let campus: Campus = form.campus.as_str().into();

    let conn = db::connect();
    db::create_user(
        &conn,
        form.email.clone(),
        form.name.clone(),
        form.password.clone(),
        form.phone.clone(),
        campus
    ).unwrap();
    let user = db::get_user_by_email(&conn, form.email.clone()).unwrap().unwrap();

    s.insert("logged_in", true).unwrap();
    s.insert("user_id", user.id.to_string()).unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}

#[get("/manage_events")]
async fn get_manage_events(s: Session) -> impl Responder {
    auth!(s);

    HttpResponse::Ok().body(
        ManageEventsTemplate {}.render().unwrap()
    )
}

#[derive(Deserialize)]
struct ManageEventForm {
    name: String,
    date: String,
    time: String,
    address1: String,
    address2: Option<String>,
    city: String,
    state: String,
    zipcode: String
}

#[post("/manage_events")]
async fn post_manage_events(s: Session, form: web::Form<ManageEventForm>) -> impl Responder {
    auth!(s);

    let time = NaiveDateTime::parse_from_str(
        &format!("{} {}", form.date, form.time), "%Y-%m-%d %H:%M"
    ).unwrap();

    let id: String = s.get("user_id").unwrap().unwrap();
    let id = Uuid::parse_str(id.as_str()).unwrap();

    let conn = db::connect();
    db::create_event(
        &conn,
        form.name.clone(),
        time,
        form.address1.clone(),
        form.address2.clone().unwrap_or("".to_string()),
        form.city.clone(),
        form.state.clone(),
        form.zipcode.clone(),
        id
    ).unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}

#[get("/reset")]
async fn get_reset_password() -> impl Responder {
    ResetPasswordTemplate {}.render().unwrap()
}

pub async fn start() -> std::io::Result<()> {
    info!("Starting Webserver");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r"))
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
            .service(post_vehicle)
            .service(get_signup)
            .service(post_signup)
            .service(get_manage_events)
            .service(post_manage_events)
            .service(get_reset_password)
            .service(get_pickup)
            .service(post_pickup)
            .service(get_seats)
            .service(post_seats)
            .service(get_css)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

