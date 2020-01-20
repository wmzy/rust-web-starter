use actix_web::{web::{Json}, get, post, put, delete, Result};
use crate::core::users::user::{User, list};

#[get("")]
pub async fn index() -> Result<Json<Vec<User>>> {
    Ok(Json(list().await?))
}