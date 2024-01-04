use crate::models::Posts;

pub async fn get_post_by_id(pool: &sqlx::MySqlPool, post_id: i32) -> Result<Posts, sqlx::Error> {
    let result = sqlx::query_as::<_, Posts>("SELECT * FROM posts where id = ?")
        .bind(post_id)
        .fetch_one(pool)
        .await?;

    Ok(result)
}