use rocket::serde::json::Json;
use crate::db_conn::db_conn;
use crate::models::{ClassScore, Res};
use crate::post_lib::*;

#[get("/")]
pub fn get_root() -> &'static str {
    "阿豪是最帅的"
}

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
            let res_str = "Successfully created student.".to_string();
            Json(Res { code: 200, msg: res_str, data: None })
        }
        Err(err) => { Json(Res { code: 400, msg: format!("Error Create: {:?}", err), data: None }) }
    }
}

#[delete("/deleteStu/<id>")]
pub async fn delete_stu_by_id(id: i32) -> Json<Res<String>> {
    let pool = db_conn().await.unwrap();
    match delete_stu(&pool, id).await {
        Ok(..) => {
            let res_str = "Successfully deleted student.".to_string();
            Json(Res { code: 200, msg: res_str, data: None })
        }
        Err(err) => {
            Json(Res {
                code: 400,
                msg: format!("Error delete student: {:?}", err),
                data: None,
            })
        }
    }
}

#[post("/updateStu", data = "<form>")]
pub async fn update_student(form: rocket::form::Form<ClassScore>) -> Json<Res<String>> {
    let pool = db_conn().await.unwrap();
    let form_data = form.into_inner();

    match update_stu(&pool, form_data).await {
        Ok(..) => {
            let res_str = "Successfully update student.".to_string();
            Json(Res { code: 200, msg: res_str, data: None })
        }
        Err(err) => { Json(Res { code: 400, msg: format!("Error Update: {:?}", err), data: None }) }
    }
}

#[get("/getScore/<stu_num>")]
pub async fn get_score(stu_num: i32) -> Json<Res<ClassScore>> {
    let pool = db_conn().await.unwrap();

    match get_this_score(&pool, stu_num).await {
        Ok(res) => { Json(Res { code: 200, msg: "Success".to_string(), data: Some(res) }) }
        Err(err) => { Json(Res { code: 400, msg: format!("Error Get: {:?}", err), data: None }) }
    }
}

#[catch(422)]
pub fn unprocessable_entity() -> Json<Res<String>> {
    Json(Res {
        code: 422,
        msg: "Unprocessable Entity: Invalid request data".to_string(),
        data: None,
    })
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_root,
        get_all_score,
        delete_stu_by_id,
        create_student,
        update_student,
        get_score,
    ]
}