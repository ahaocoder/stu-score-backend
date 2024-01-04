use crate::models::*;

// pub async fn get_post_by_id(pool: &sqlx::MySqlPool, post_id: i32) -> Result<Posts, sqlx::Error> {
//     let result = sqlx::query_as::<_, Posts>("SELECT * FROM posts where id = ?")
//         .bind(post_id)
//         .fetch_one(pool)
//         .await?;
//
//     Ok(result)
// }

pub async fn get_all_scores(pool: &sqlx::MySqlPool) -> Result<Vec<ClassScore>, sqlx::Error> {
    let scores = sqlx::query_as!(ClassScore, "SELECT * FROM class_score")
        .fetch_all(pool)
        .await?;

    Ok(scores)
}

// pub async fn create_stu()
//
// pub async fn delete_stu()
//
// pub async fn update_stu()
//
// pub async fn login_admin()
//
// pub async fn login_stu()
//
// pub async fn get_this_score()
