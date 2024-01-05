#[macro_use]
extern crate rocket;

mod db_conn;
mod models;
mod post_lib;
mod routes;
mod config;

use rocket::serde::json::Json;
use routes::get_routes;
use crate::models::Res;

#[catch(422)]
fn unprocessable_entity() -> Json<Res<String>> {
    Json(Res {
        code: 422,
        msg: "Unprocessable Entity: Invalid request data".to_string(),
        data: None,
    })
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api", get_routes())
        .register("/",catchers![unprocessable_entity])
}


