use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Posts {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: String,
}

// pub struct ResData<T>{
//     pub code: i32,
//     pub message: String,
//     pub data: Option<T>,
// }