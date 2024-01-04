use sqlx::mysql::MySqlPoolOptions;
use config::{Config, File};
use crate::config::AppConfig;

pub async fn db_conn() -> Result<sqlx::MySqlPool, sqlx::Error> {
    let config_ = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("构建配置错误");
    let config: AppConfig = config_.try_deserialize().expect("反序列化配置文件错误");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&*config.database_url)
        .await?;
    Ok(pool)
}
