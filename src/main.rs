mod models;
mod handlers;
mod error;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use crate::handlers::{create_item, delete_item, read_items, TodoItem, update_item};


// Initialize logger
fn init_logger() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let todo_items = web::Data::new(Mutex::new(Vec::new() as Vec<TodoItem>));

    HttpServer::new(move || {
        App::new()
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
