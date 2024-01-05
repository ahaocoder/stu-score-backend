use sqlx::mysql::MySqlQueryResult;
use crate::models::*;
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    // 你可以根据需要添加更多的声明
}

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

pub async fn login_admin(pool: &sqlx::MySqlPool, user: User) -> Result<Option<String>, sqlx::Error> {
    let admin_query = sqlx::query!(
        "SELECT * FROM user WHERE username = ?",
        user.username
    )
        .fetch_optional(pool)
        .await?;

    if let Some(admin) = admin_query {
        // If the user exists, check if the provided password matches the stored one
        if user.password == admin.password {
            // 生成 JWT 令牌
            let claims = Claims {
                sub: user.username.to_string(),
            };

            let secret_key = "your_secret_key"; // 用于签名的秘钥，请替换成实际的秘钥
            let token_result: Result<String, jsonwebtoken::errors::Error> = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret_key.as_ref()),
            );

            // 处理生成令牌的结果
            match token_result {
                Ok(token) => return Ok(Some(token)),
                Err(_) => return Ok(None), // 生成令牌失败，返回 None 或其他适当的值
            }
        }
    }

    Ok(None)
}

pub async fn login_stu(pool: &sqlx::MySqlPool, stu_num: i32, password: &str) -> Result<Option<String>, sqlx::Error> {
    let stu_query = sqlx::query!(
        "SELECT COUNT(*) as count FROM class_score WHERE stu_num = ?",
        stu_num,
    )
        .fetch_one(pool)
        .await?;

    if stu_query.count > 0 {
        let the_password = "123456";

        if password == the_password {
            // 生成 JWT 令牌
            let claims = Claims {
                sub: stu_num.to_string(),
            };

            let secret_key = "your_secret_key"; // 用于签名的秘钥，请替换成实际的秘钥
            let token_result: Result<String, jsonwebtoken::errors::Error> = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret_key.as_ref()),
            );

            // 处理生成令牌的结果
            match token_result {
                Ok(token) => return Ok(Some(token)),
                Err(_) => return Ok(None), // 生成令牌失败，返回 None 或其他适当的值
            }
        }
    }

    Ok(None)
}

pub async fn get_this_score(pool: &sqlx::MySqlPool, stu_num: i32) -> Result<ClassScore, sqlx::Error> {
    let res = sqlx::query_as!(ClassScore, "SELECT * FROM class_score where stu_num = ?", stu_num)
        .fetch_one(pool)
        .await?;

    Ok(res)
}
