table! {
    tenants (id) {
        id -> Integer,
        email -> Varchar,
        name -> Varchar,
        username -> Varchar,
        role -> Varchar,
        password -> Varchar,
    }
}

table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    auth_info,
    tenants,
)