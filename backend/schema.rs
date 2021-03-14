table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        age -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    auth_infos,
    users,
);