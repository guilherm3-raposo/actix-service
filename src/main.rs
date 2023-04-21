mod db;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use crate::db::{create, run_migrations};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    create().await;
    run_migrations().await;

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
