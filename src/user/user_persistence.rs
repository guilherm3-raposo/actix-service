use crate::{build_user, db::get_connection};
use sqlx::Error;
use std::str::FromStr;

use super::user_model::{Person, Role, User};

pub async fn get_by_id(id: i32) -> Result<User, Error> {
    let res = sqlx::query_file!("src/user/queries/select_by_id.sql", id)
        .fetch_one(&get_connection().await?)
        .await;

    match res {
        Ok(row) => Ok(build_user!(row)),

        Err(err) => Err(err),
    }
}

pub async fn get_all() -> Result<Vec<User>, Error> {
    let res = sqlx::query_file!("src/user/queries/select_all.sql")
        .fetch_all(&get_connection().await?)
        .await;

    match res {
        Ok(rows) => {
            let mut users: Vec<User> = Vec::new();

            for row in rows {
                users.push(build_user!(row));
            }

            Ok(users)
        }

        Err(err) => Err(err),
    }
}

pub async fn insert_one(user: User) -> Result<User, Error> {
    let db = get_connection().await;

    match db {
        Ok(conn) => {
            let res = sqlx::query_file!(
                "src/user/queries/insert_one.sql",
                user.id,
                user.username,
                user.email
            )
            .execute(&conn)
            .await;
            match res {
                Ok(row) => Ok(User {
                    id: i32::try_from(row.last_insert_id()).unwrap_or(i32::MAX),
                    username: user.username,
                    email: user.email,
                    uuid: user.uuid,
                    person: Person::empty(),
                    roles: Vec::new(),
                }),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn update(user: User) -> Result<bool, Error> {
    let db = get_connection().await;

    match db {
        Ok(conn) => {
            let _ = sqlx::query_file!(
                "src/user/queries/update.sql",
                user.username,
                user.email,
                user.uuid,
                user.id
            )
            .execute(&conn)
            .await;
            Ok(true)
        }
        Err(err) => Err(err),
    }
}
