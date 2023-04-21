use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
}

impl User {
    pub fn from_new_user(nu: NewUser) -> User {
        User {
            id: 0,
            email: nu.email,
            username: nu.username,
            uuid: None
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    firstname: Option<String>,
    lastname: Option<String>,
}