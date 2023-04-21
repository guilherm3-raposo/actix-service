use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result, Scope};

use super::user_service as s;
use crate::user::user_model::{NewUser, User};

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
    let u = User::from_new_user(new_user.0);

    let res = s::insert_one(u).await;

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
async fn delete(id: web::Json<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", id))
}

pub fn scope() -> Scope {
    web::scope("/users")
        .service(get_by_id)
        .service(get_all)
        .service(insert)
        .service(put)
        .service(delete)
}
