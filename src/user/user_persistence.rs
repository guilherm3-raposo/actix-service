use crate::db::get_connection;
use sqlx::Error;

use super::user_model::{User};

pub async fn get_by_id(id: i32) -> Result<User, Error> {
    sqlx::query_file_as!(User, "src/user/queries/select_by_id.sql", id).fetch_one(&get_connection().await?).await
}

pub async fn get_all() -> Result<Vec<User>, Error> {
    sqlx::query_file_as!(User, "src/user/queries/select_all.sql").fetch_all(&get_connection().await?).await
}

pub async fn insert_one(user: User) -> Result<User, Error> {
    let db = get_connection().await;

    match db {
        Ok(conn) => {
            let res = sqlx::query_file!("src/user/queries/insert_one.sql", user.id, user.username, user.email).execute(&conn).await;
            match res {
                Ok(row) => {
                    Ok(User {
                        id: i32::try_from(row.last_insert_id()).unwrap_or(i32::MAX),
                        username: user.username,
                        email: user.email,
                        uuid: user.uuid
                    })
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }

}

pub async fn update(user: User) -> Result<bool, Error> {
    let db = get_connection().await;
    
    match db {
        Ok(conn) => {
            let _ = sqlx::query_file!("src/user/queries/update.sql", user.username, user.email, user.uuid, user.id).execute(&conn).await;
            Ok(true)
        },
        Err(err) => Err(err)
    }

}