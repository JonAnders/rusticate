mod models;
mod handlers;
mod error;
mod db;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use crate::db::init_pool;
use crate::handlers::{create_item, delete_item, read_items, update_item};


// Initialize logger
fn init_logger() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let todo_items = web::Data::new(init_pool(&database_url));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(todo_items.clone())
            .route("/", web::get().to(index))
            .route("/items", web::post().to(create_item))
            .route("/items", web::get().to(read_items))
            .route("/items/{item_id}", web::put().to(update_item))
            .route("/items/{item_id}", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rusticate!")
}
