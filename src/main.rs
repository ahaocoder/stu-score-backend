#[macro_use]
extern crate rocket;

mod db_conn;
mod models;
mod post_lib;
mod routes;
mod config;

use rocket_cors::{CorsOptions};
use routes::get_routes;


#[launch]
async fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allow_credentials(true)
        .to_cors()
        .expect("CorsOptions failed to convert to Cors");

    rocket::build()
        .mount("/api", get_routes())
        .attach(cors)
        .register("/", catchers![routes::unprocessable_entity, routes::not_found, routes::unsupported_media_type])
}


