use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub id: u64,
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
    pub date: String,
    pub id: String,
}
