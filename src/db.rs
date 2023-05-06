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

#[cfg(test)]
mod tests {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use serial_test::serial;
    use super::*;
    use crate::models::TodoItem;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    fn create_in_memory_database() -> Pool {
        init_pool("file::memory:?cache=shared")
    }

    pub fn run_migrations(connection: &mut SqliteConnection) {
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    }

    #[test]
    #[serial]
    fn test_create_item() {
        let mut connection = create_in_memory_database().get().unwrap();
        run_migrations(&mut connection);
        let item = TodoItem {
            id: 1,
            title: "Test item".to_string(),
            description: "Test description".to_string(),
            completed: false,
        };

        let created_rows = create_item(&mut connection, &item).unwrap();
        assert_eq!(created_rows, 1);
    }

    #[test]
    #[serial]
    fn test_read_items() {
        let mut connection = create_in_memory_database().get().unwrap();
        run_migrations(&mut connection);
        let item = TodoItem {
            id: 1,
            title: "Test item".to_string(),
            description: "Test description".to_string(),
            completed: false,
        };

        create_item(&mut connection, &item).unwrap();
        let items = read_items(&mut connection).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], item);
    }

    #[test]
    #[serial]
    fn test_update_item() {
        let mut connection = create_in_memory_database().get().unwrap();
        run_migrations(&mut connection);
        let item = TodoItem {
            id: 1,
            title: "Test item".to_string(),
            description: "Test description".to_string(),
            completed: false,
        };

        create_item(&mut connection, &item).unwrap();
        let updated_item = TodoItem {
            id: 1,
            title: "Updated item".to_string(),
            description: "Updated description".to_string(),
            completed: true,
        };

        let updated_rows = update_item(&mut connection, 1, &updated_item).unwrap();
        assert_eq!(updated_rows, 1);

        let items = read_items(&mut connection).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], updated_item);
    }

    #[test]
    #[serial]
    fn test_delete_item() {
        let mut connection = create_in_memory_database().get().unwrap();
        run_migrations(&mut connection);
        let item = TodoItem {
            id: 1,
            title: "Test item".to_string(),
            description: "Test description".to_string(),
            completed: false,
        };

        create_item(&mut connection, &item).unwrap();
        let deleted_rows = delete_item(&mut connection, 1).unwrap();
        assert_eq!(deleted_rows, 1);

        let items = read_items(&mut connection).unwrap();
        assert_eq!(items.len(), 0);
    }
}
