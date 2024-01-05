#[macro_use]
extern crate rocket;

mod db_conn;
mod models;
mod post_lib;
mod routes;
mod config;

use routes::get_routes;


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api", get_routes())
        .register("/", catchers![routes::unprocessable_entity, routes::not_found])
}


