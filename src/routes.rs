use rocket::get;
use crate::db_conn::db_conn;
use crate::post_lib::get_post_by_id;

#[get("/post/<id>")]
pub async fn get_post(id: i32) -> String {
    let pool = db_conn().await.unwrap();
    match get_post_by_id(&pool, id).await {
        Ok(post) => format!("Post: {:?}", post),
        Err(err) => format!("Error fetching posts: {:?}", err),
    }
}