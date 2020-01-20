use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    username: &'static str,
}

pub async fn list() -> Result<Vec<User>, io::Error> {
    Ok(vec![User { username: "wmzy"}])
}