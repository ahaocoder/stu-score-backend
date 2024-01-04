use sqlx::mysql::MySqlPoolOptions;

pub async fn db_conn() -> Result<sqlx::MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:12345678@localhost/school")
        .await?;
    Ok(pool)
}
