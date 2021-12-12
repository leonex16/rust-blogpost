table! {
    notes (id) {
        id -> Int4,
        description -> Text,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        alias -> Text,
        created_at -> Timestamp,
    }
}

joinable!(notes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
