use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result, Scope};

use super::user_service as s;
use crate::user::user_model::{NewUser, Role, User, UserRoleUpdate};
use strum::IntoEnumIterator;

#[get("/sample")]
async fn sample() -> Result<impl Responder> {
    Ok(web::Json(User::empty()))
}

#[get("/all")]
async fn get_all() -> Result<impl Responder> {
    let res = s::get_all().await;

    match res {
        Ok(u) => Ok(web::Json(u)),
        Err(err) => Err(error::ErrorNotFound(err)),
    }
}

#[get("/one/{id}")]
async fn get_by_id(path_params: web::Path<i32>) -> Result<impl Responder> {
    let res = s::get_by_id(path_params.into_inner()).await;

    match res {
        Ok(u) => Ok(web::Json(u)),
        Err(err) => Err(error::ErrorNotFound(err)),
    }
}

#[post("/")]
async fn insert(new_user: web::Json<NewUser>) -> Result<impl Responder> {
    let res = s::insert_one(new_user.0).await;

    match res {
        Ok(u) => Ok(web::Json(u)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

#[put("/")]
async fn put(user: web::Json<User>) -> Result<impl Responder> {
    let res = s::update(user.0).await;

    match res {
        Ok(_) => Ok(web::Json(true)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

#[delete("/{id}")]
async fn delete(id: web::Json<i32>) -> impl Responder {
    match s::delete_by_id(id.0).await {
        Ok(one_row) => {
            if one_row {
                HttpResponse::Ok()
            } else {
                HttpResponse::BadRequest()
            }
        }
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[get("/roles")]
async fn get_roles() -> Result<impl Responder> {
    let mut arr: Vec<String> = vec![];

    Role::iter().for_each(|r| arr.push(r.to_string()));

    Ok(web::Json(arr))
}

#[put("/roles")]
async fn put_roles(user: web::Json<UserRoleUpdate>) -> Result<impl Responder> {
    print!("{:?}", user.0);
    let res = s::update_user_roles(user.0).await;

    match res {
        Ok(_) => Ok(web::Json(true)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

pub fn scope() -> Scope {
    web::scope("/users")
        .service(sample)
        .service(get_by_id)
        .service(get_all)
        .service(insert)
        .service(put)
        .service(delete)
        .service(get_roles)
        .service(put_roles)
}
