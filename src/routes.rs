use rocket::get;
use rocket::serde::json::Json;
use crate::db_conn::db_conn;
use crate::models::{Posts, Res};
use crate::post_lib::get_post_by_id;

#[get("/post/<id>")]
pub async fn get_post(id: i32) -> Json<Res<Posts>> {
    let pool = db_conn().await.unwrap();
    match get_post_by_id(&pool, id).await {
        Ok(post) => { Json(Res { code: 200, msg: "Success".to_string(), data: Some(post) }) }
        Err(err) => { Json(Res { code: 400, msg: format!("Error fetching posts: {:?}", err), data: None }) }
    }
}