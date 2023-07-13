use crate::{build_user, db::get_connection};
use sqlx::{Error, QueryBuilder};
use std::str::FromStr;

use super::user_model::{Person, Role, User, UserRoleUpdate};

pub async fn get_by_id(id: i32) -> Result<User, Error> {
    let row = sqlx::query_file!("src/user/queries/select_by_id.sql", id)
        .fetch_one(&get_connection().await?)
        .await?;

    Ok(build_user!(row))
}

pub async fn get_all() -> Result<Vec<User>, Error> {
    let rows = sqlx::query_file!("src/user/queries/select_all.sql")
        .fetch_all(&get_connection().await?)
        .await?;

    let mut users: Vec<User> = Vec::new();

    for row in rows {
        users.push(build_user!(row));
    }

    Ok(users)
}

pub async fn insert_one(user: User) -> Result<User, Error> {
    let conn = get_connection().await?;

    let row = sqlx::query_file!(
        "src/user/queries/insert_one.sql",
        user.username,
        user.email,
        user.uuid,
    )
    .execute(&conn)
    .await?;

    let id = i32::try_from(row.last_insert_id()).unwrap_or(i32::MAX);

    let _ = sqlx::query_file!(
        "src/user/queries/insert_one_person.sql",
        id,
        user.person.firstname,
        user.person.lastname,
        user.person.avatar,
    )
    .execute(&conn)
    .await;

    Ok(User {
        id,
        username: user.username,
        email: user.email,
        uuid: user.uuid,
        person: Person {
            firstname: user.person.firstname,
            lastname: user.person.lastname,
            avatar: user.person.avatar,
        },
        roles: Vec::new(),
    })
}

pub async fn update(user: User) -> Result<bool, Error> {
    let conn = get_connection().await?;

    let res = sqlx::query_file!(
        "src/user/queries/update.sql",
        user.username,
        user.email,
        user.uuid,
        user.person.firstname,
        user.person.lastname,
        user.person.avatar,
        user.id
    )
    .execute(&conn)
    .await?;

    Ok(res.rows_affected() == 1)
}

pub async fn delete_by_id(id: i32) -> Result<u64, Error> {
    let conn = get_connection().await?;

    let res = sqlx::query_file!("src/user/queries/delete.sql", id)
        .execute(&conn)
        .await?;

    Ok(res.rows_affected())
}

pub async fn update_user_roles(user: UserRoleUpdate) -> Result<(), Error> {
    let conn = get_connection().await?;

    let id = user.id;

    let _ = sqlx::query!("DELETE FROM `role` WHERE user_id = ?", id)
        .execute(&conn)
        .await?;

    let mut roles = user.roles;
    roles.push(Role::USER);

    let _ = QueryBuilder::new("INSERT IGNORE INTO `role` (`user_id`, `value`)")
        .push_values(roles, |mut b, role| {
            b.push_bind(id).push_bind(role.to_string());
        })
        .build()
        .execute(&conn)
        .await?;

    Ok(())
}
