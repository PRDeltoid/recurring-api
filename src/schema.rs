table! {
    chore_entries (id) {
        id -> Nullable<Int4>,
        date -> Date,
        choreid -> Int4,
        userid -> Int4,
    }
}

table! {
    chores (id) {
        id -> Nullable<Int4>,
        name -> Text,
        interval -> Int4,
        userid -> Int4,
    }
}

table! {
    users (id) {
        id -> Nullable<Int4>,
        username -> Text,
    }
}

joinable!(chore_entries -> chores (choreid));
joinable!(chore_entries -> users (userid));
joinable!(chores -> users (userid));

allow_tables_to_appear_in_same_query!(
    chore_entries,
    chores,
    users,
);
