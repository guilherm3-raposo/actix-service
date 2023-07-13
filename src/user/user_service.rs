use sqlx::Error;

use super::user_model::{NewUser, User, UserRoleUpdate};
use crate::user::user_persistence as p;

pub async fn get_by_id(id: i32) -> Result<User, Error> {
    p::get_by_id(id).await
}

pub async fn get_all() -> Result<Vec<User>, Error> {
    p::get_all().await
}

pub async fn insert_one(user: NewUser) -> Result<User, Error> {
    let u = User::from_new_user(user);
    p::insert_one(u).await
}

pub async fn update(user: User) -> Result<bool, Error> {
    p::update(user).await
}

pub async fn delete_by_id(id: i32) -> Result<bool, Error> {
    let rows = p::delete_by_id(id).await?;

    match rows {
        1 => Ok(true),
        0 => Err(Error::RowNotFound),
        _ => Ok(false),
    }
}

pub async fn update_user_roles(user: UserRoleUpdate) -> Result<(), Error> {
    p::update_user_roles(user).await
}
