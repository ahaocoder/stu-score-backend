use rocket::tokio;
use sqlx::mysql::MySqlPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// #[async_std::main] // Requires the `attributes` feature of `async-std`
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:12345678@localhost/school").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT id from posts where title = ?")
        .bind("2")
        .fetch_one(&pool).await?;

    println!("{:?}", row);

    Ok(())
}