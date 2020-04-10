table! {
    day_data (id) {
        id -> Int4,
        day -> Date,
        mood_value -> Text,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    day_data,
    user,
);
