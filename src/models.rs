use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub status: Option<i8>,
}

#[derive(Debug, FromRow, Serialize, FromForm)]
pub struct ClassScore {
    pub id: Option<i32>,
    pub stu_num: i32,
    pub name: String,
    pub math: f32,
    pub english: f32,
    pub chinese: f32,
    pub frontend: f32,
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