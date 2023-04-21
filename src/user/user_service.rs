use sqlx::Error;

use crate::user::user_persistence as p;
use super::user_model::User;

pub async fn get_by_id(id: i32) -> Result<User, Error> {
    p::get_by_id(id).await
}

pub async fn get_all() -> Result<Vec<User>, Error> {
    p::get_all().await
}

pub async fn insert_one(user: User) -> Result<User, Error> {
    p::insert_one(user).await
}

pub async fn update(user: User) -> Result<bool, Error> {
    p::update(user).await
}
