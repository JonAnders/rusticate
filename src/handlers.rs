use crate::error::TodoApiError;
use crate::models::TodoItem;
use crate::db;
use actix_web::{web, HttpResponse, Responder};
use diesel::result::Error as DieselError;

// Create a new to-do item and add it to the database
pub async fn create_item(
    item: web::Json<TodoItem>,
    pool: web::Data<db::Pool>,
) -> Result<impl Responder, TodoApiError> {
    let mut connection = pool.get().map_err(TodoApiError::from)?;

    let new_item = item.into_inner();
    let _ = db::create_item(&mut connection, &new_item)?;

    Ok(HttpResponse::Created().finish())
}

// Retrieve all to-do items from the database
pub async fn read_items(pool: web::Data<db::Pool>) -> Result<HttpResponse, TodoApiError> {
    let mut connection = pool.get().map_err(TodoApiError::from)?;
    let items = db::read_items(&mut connection)?;

    Ok(HttpResponse::Ok().json(items))
}

// Update an existing to-do item in the database by its ID
pub async fn update_item(
    item_id: web::Path<i32>,
    item: web::Json<TodoItem>,
    pool: web::Data<db::Pool>,
) -> Result<impl Responder, TodoApiError> {
    let mut connection = pool.get().map_err(TodoApiError::from)?;
    let updated_item = item.into_inner();

    match db::update_item(&mut connection, *item_id, &updated_item) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(DieselError::NotFound) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Err(TodoApiError::from("Internal server error")),
    }
}

// Delete a to-do item from the database by its ID
pub async fn delete_item(
    item_id: web::Path<i32>,
    pool: web::Data<db::Pool>,
) -> Result<impl Responder, TodoApiError> {
    let mut connection = pool.get().map_err(TodoApiError::from)?;

    match db::delete_item(&mut connection, *item_id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(DieselError::NotFound) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Err(TodoApiError::from("Internal server error")),
    }
}
