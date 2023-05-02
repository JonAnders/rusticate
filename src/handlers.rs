use actix_web::{web, HttpResponse, Responder, Error};
use std::sync::Mutex;
use log::{error, info};
use serde::{Deserialize, Serialize};
use crate::error::{TodoApiError, TodoApiErrorKind};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TodoItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

// Create a new to-do item and add it to the shared state
pub async fn create_item(
    item: web::Json<TodoItem>,
    todo_items: web::Data<Mutex<Vec<TodoItem>>>,
) -> Result<impl Responder, TodoApiError> {
    info!("create_item called");
    let mut items = todo_items.lock().map_err(|e| {
        let error_message = format!("Failed to lock Mutex: {}", e);
        error!("{}", error_message);
        TodoApiError { kind: TodoApiErrorKind::MutexLockError }
    })?;
    items.push(item.into_inner());
    Ok(HttpResponse::Created().finish())
}

// Retrieve all to-do items from the shared state
pub async fn read_items(
    todo_items: web::Data<Mutex<Vec<TodoItem>>>,
) -> Result<HttpResponse, TodoApiError> {
    info!("get_items called");
    let items = todo_items
        .lock()
        .map_err(|e| {
            let error_message = format!("Failed to lock Mutex: {}", e);
            error!("{}", error_message);
            TodoApiError { kind: TodoApiErrorKind::MutexLockError }
        })?;
    info!("Returning items: {:?}", items);
    Ok(HttpResponse::Ok().json(items.clone()))
}

// Update an existing to-do item in the shared state by its ID
pub async fn update_item(item_id: web::Path<u64>, item: web::Json<TodoItem>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> Result<impl Responder, Error> {
    info!("update_item called");
    let mut items = todo_items.lock().map_err(|e| {
        let error_message = format!("Failed to lock Mutex: {}", e);
        error!("{}", error_message);
        TodoApiError { kind: TodoApiErrorKind::MutexLockError }
    })?;
    for i in items.iter_mut() {
        if i.id == *item_id {
            *i = item.into_inner();
            return Ok(HttpResponse::Ok().finish());
        }
    }
    Ok(HttpResponse::NotFound().finish())
}

// Delete a to-do item from the shared state by its ID
pub async fn delete_item(item_id: web::Path<u64>, todo_items: web::Data<Mutex<Vec<TodoItem>>>) -> Result<impl Responder, Error> {
    info!("delete_item called");
    let mut items = todo_items.lock().map_err(|e| {
        let error_message = format!("Failed to lock Mutex: {}", e);
        error!("{}", error_message);
        TodoApiError { kind: TodoApiErrorKind::MutexLockError }
    })?;
    if let Some(index) = items.iter().position(|i| i.id == *item_id) {
        items.remove(index);
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
