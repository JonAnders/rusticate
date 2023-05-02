use crate::models::TodoItem;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

// Create
pub async fn create_item(item: web::Json<TodoItem>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
    let mut items = todo_items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Created().finish()
}

// Read
pub async fn read_items(todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
    let items = todo_items.lock().unwrap();
    web::Json(items.clone())
}

// Update
pub async fn update_item(item_id: web::Path<u64>, item: web::Json<TodoItem>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
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
pub async fn delete_item(item_id: web::Path<u64>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> impl Responder {
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