use crate::models::TodoItem;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create the pool.")
}

pub fn create_item(connection: &mut SqliteConnection, item: &TodoItem) -> QueryResult<usize> {
    use crate::schema::todo_items;

    diesel::insert_into(todo_items::table)
        .values(item)
        .execute(connection)
}

pub fn read_items(connection: &mut SqliteConnection) -> QueryResult<Vec<TodoItem>> {
    use crate::schema::todo_items::dsl::*;

    todo_items.load::<TodoItem>(connection)
}

pub fn update_item(connection: &mut SqliteConnection, item_id: i32, updated_item: &TodoItem) -> QueryResult<usize> {
    use crate::schema::todo_items::dsl::*;

    diesel::update(todo_items.filter(id.eq(item_id)))
        .set((
            title.eq(&updated_item.title),
            description.eq(&updated_item.description),
            completed.eq(&updated_item.completed),
        ))
        .execute(connection)
}

pub fn delete_item(connection: &mut SqliteConnection, item_id: i32) -> QueryResult<usize> {
    use crate::schema::todo_items::dsl::*;

    diesel::delete(todo_items.filter(id.eq(item_id))).execute(connection)
}
