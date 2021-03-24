table! {
    tenants (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
    }
}
