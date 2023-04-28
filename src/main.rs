mod models;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;
use crate::models::TodoItem;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rusticate!")
}

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

// Create
async fn create_item(item: web::Json<TodoItem>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
    let mut items = todo_items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Created().finish()
}

// Read
async fn read_items(todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
    let items = todo_items.lock().unwrap();
    let json = serde_json::to_string(&*items).unwrap();
    HttpResponse::Ok().body(json)
}

// Update
async fn update_item(item_id: web::Path<u64>, item: web::Json<TodoItem>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
    let mut items = todo_items.lock().unwrap();
    let item_id = item_id.into_inner();

    for todo_item in items.iter_mut() {
        if todo_item.id == item_id {
            *todo_item = item.into_inner();
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::NotFound().finish()
}

// Delete
async fn delete_item(item_id: web::Path<u64>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
    let mut items = todo_items.lock().unwrap();
    let item_id = item_id.into_inner();

    let original_len = items.len();
    items.retain(|todo_item| todo_item.id != item_id);

    if original_len == items.len() {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::NoContent().finish()
    }
}
