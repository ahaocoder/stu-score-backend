use sqlx::mysql::MySqlQueryResult;
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

pub async fn create_stu(pool: &sqlx::MySqlPool, form_data: ClassScore) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!(
        "INSERT INTO class_score (stu_num, name, math, english, chinese, frontend) VALUES (?, ?, ?, ?, ?, ?)",
        form_data.stu_num,
        form_data.name,
        form_data.math,
        form_data.english,
        form_data.chinese,
        form_data.frontend,
    )
        .execute(pool)
        .await?;

    Ok(res.rows_affected() > 0)
}

pub async fn delete_stu(pool: &sqlx::MySqlPool, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
    let res = sqlx::query!("DELETE FROM class_score WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(res)
}
//
// pub async fn update_stu()
//
// pub async fn login_admin()
//
// pub async fn login_stu()
//
// pub async fn get_this_score()
