use chrono::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub fullname: String,
    pub role: String,
}

pub struct UserLogin {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub struct Event {
    pub name: String,
    pub date: DateTime<Utc>,
    pub id: String,
    pub owner_id: String,
}
