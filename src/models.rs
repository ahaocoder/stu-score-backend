use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: i8,
}

#[derive(Debug, FromRow, Serialize)]
pub struct ClassScore {
    pub id: i64,
    pub stu_num: i32,
    pub name: String,
    pub math: f64,
    pub english: f64,
    pub chinese: f64,
    pub frontend: f64,
}

#[derive(Debug, Serialize)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub port: u16,
}