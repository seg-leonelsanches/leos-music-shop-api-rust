use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use r2d2_sqlite::{self, SqliteConnectionManager};

#[macro_use] extern crate diesel;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use serde::Deserialize;

mod actions;
mod db;
mod models;
mod routes;
mod schema;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/queries")]
async fn queries(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Diesel pool
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::hello::hello)
            .service(routes::hello::echo)
            .service(routes::microphones::get_all_microphones)
            .service(routes::microphones::get_microphone)
            .service(routes::microphones::add_microphone)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 7070))?
    .run()
    .await
}