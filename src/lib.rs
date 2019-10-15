#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate serde_derive;
#[macro_use]
extern crate rocket_cors;
#[macro_use]
extern crate diesel;

use rocket::http::Method;
use rocket_contrib::json::JsonValue;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
mod db;
mod models;
mod routes;
mod schema;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8000",
        "http://localhost:3000",
        "http://0.0.0.0:8000",
        "http://0.0.0.0:3000",
        "http://127.0.0.1:8000",
        "http://127.0.0.1:3000",
    ]);
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/")]
fn index() -> &'static str {
    "Server up and running"
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status":"error",
        "reason":"Resource was not found"
    })
}

pub fn start() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![index, routes::users::get_user])
        .attach(db::Conn::fairing())
        .attach(make_cors())
        .register(catchers![not_found])
}
