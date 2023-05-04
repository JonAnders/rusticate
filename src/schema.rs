use diesel::table;

table! {
    todo_items (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        completed -> Bool,
    }
}
