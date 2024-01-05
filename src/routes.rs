use rocket::serde::json::Json;
use crate::db_conn::db_conn;
use crate::models::{ClassScore, Res};
use crate::post_lib::*;

#[get("/")]
pub fn get_root() -> &'static str {
    "阿豪是最帅的"
}

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

#[post("/createStu", data = "<form>")]
pub async fn create_student(form: rocket::form::Form<ClassScore>) -> Json<Res<String>> {
    let pool = db_conn().await.unwrap();
    let form_data = form.into_inner();

    match create_stu(&pool, form_data).await {
        Ok(..) => {
            let res_str = "Successfully deleted student.".to_string();
            Json(Res { code: 200, msg: res_str, data: None })
        }
        Err(err) => { Json(Res { code: 400, msg: format!("Error Delete: {:?}", err), data: None }) }
    }
}

#[delete("/deleteStu/<id>")]
pub async fn delete_stu_by_id(id: i32) -> Json<Res<String>> {
    let pool = db_conn().await.unwrap();
    match delete_stu(&pool, id).await {
        Ok(..) => {
            let res_str = "Successfully created student.".to_string();
            Json(Res { code: 200, msg: res_str, data: None })
        }
        Err(err) => {
            Json(Res {
                code: 400,
                msg: format!("Error creating student: {:?}", err),
                data: None,
            })
        }
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_root,
        get_all_score,
        delete_stu_by_id,
        create_student,
    ]
}