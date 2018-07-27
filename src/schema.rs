table! {
    chores (id) {
        id -> Nullable<Int4>,
        name -> Text,
        interval -> Int4,
    }
}

table! {
    users (id) {
        id -> Nullable<Int4>,
        username -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    chores,
    users,
);
