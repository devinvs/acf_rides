use log::info;
use actix_web::{App, HttpServer, Responder, get, HttpResponse, post, web};
use actix_web::cookie::Key;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use serde::Deserialize;
use uuid::Uuid;
use crate::db;
use crate::models::{Campus, Event, Vehicle};
use askama::Template;

// Templates
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: String
}

#[derive(Template)]
#[template(path = "drive_or_ride.html")]
struct DriveOrRideTemplate {}

#[derive(Template)]
#[template(path = "driver_summary.html")]
struct DriverSummaryTemplate {}

#[derive(Template)]
#[template(path = "events.html")]
struct EventsTemplate {
    events: Vec<Event>
}

#[derive(Template)]
#[template(path = "manage_events.html")]
struct ManageEventsTemplate {}

#[derive(Template)]
#[template(path = "reset_password.html")]
struct ResetPasswordTemplate {}

#[derive(Template)]
#[template(path = "rider_summary.html")]
struct RiderSummaryTemplate {}

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

#[get("/")]
async fn get_root(s: Session) -> impl Responder {
    auth!(s);
    HttpResponse::Ok().body(
        DriveOrRideTemplate {}.render().unwrap()
    )
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

#[get("/events")]
async fn get_events(s: Session) -> impl Responder {
    auth!(s);

    let conn = db::connect();
    let events = db::get_events(&conn).unwrap();

    HttpResponse::Ok().body(
        EventsTemplate {
            events
        }.render().unwrap()
    )
}

#[get("/vehicles")]
async fn get_vehicles(s: Session) -> impl Responder {
    auth!(s);

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

#[get("/reset")]
async fn get_reset_password() -> impl Responder {
    ResetPasswordTemplate {}.render().unwrap()
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
            .service(post_vehicle)
            .service(get_signup)
            .service(post_signup)
            .service(get_manage_events)
            .service(get_reset_password)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
