#[macro_use]
extern crate rocket;

mod db_conn;
mod models;
mod post_lib;
mod routes;
mod config;


#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::get_post])
}


