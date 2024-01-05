use sqlx::mysql::MySqlQueryResult;
use crate::models::*;

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

pub async fn update_stu(pool: &sqlx::MySqlPool, form_data: ClassScore) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!(
        "UPDATE class_score SET stu_num = ?, name = ?, math = ?, english = ?, chinese = ?, frontend = ? WHERE id = ?",
        form_data.stu_num,
        form_data.name,
        form_data.math,
        form_data.english,
        form_data.chinese,
        form_data.frontend,
        form_data.id,
    )
        .execute(pool)
        .await?;

    Ok(res.rows_affected() > 0)
}

// pub async fn login_admin()
//
// pub async fn login_stu()

pub async fn get_this_score(pool: &sqlx::MySqlPool, stu_num: i32) -> Result<ClassScore, sqlx::Error> {
    let res = sqlx::query_as!(ClassScore, "SELECT * FROM class_score where stu_num = ?", stu_num)
        .fetch_one(pool)
        .await?;

    Ok(res)
}
