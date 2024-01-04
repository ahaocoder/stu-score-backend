use rocket::serde::json::Json;
use crate::db_conn::db_conn;
use crate::models::{ClassScore, Res};
use crate::post_lib::*;

// #[get("/post/<id>")]
// pub async fn get_post(id: i32) -> Json<Res<Posts>> {
//     let pool = db_conn().await.unwrap();
//     match get_post_by_id(&pool, id).await {
//         Ok(post) => { Json(Res { code: 200, msg: "Success".to_string(), data: Some(post) }) }
//         Err(err) => { Json(Res { code: 400, msg: format!("Error fetching posts: {:?}", err), data: None }) }
//     }
// }

#[get("/getAllScores")]
pub async fn get_all_score() -> Json<Res<Vec<ClassScore>>> {
    let pool = db_conn().await.unwrap();
    match get_all_scores(&pool).await {
        Ok(res) => { Json(Res { code: 200, msg: "Success".to_string(), data: Some(res) }) }
        Err(err) => { Json(Res { code: 400, msg: format!("Error fetching: {:?}", err), data: None }) }
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_all_score,
    ]
}