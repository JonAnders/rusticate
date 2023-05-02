mod models;
mod handlers;

use crate::models::TodoItem;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;
use handlers::{create_item, delete_item, read_items, update_item};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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