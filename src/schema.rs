table! {
    applets (id) {
        id -> Int4,
        name -> Varchar,
        position_x -> Int4,
        position_y -> Int4,
        width -> Int4,
        height -> Int4,
        editable -> Bool,
        applet_data -> Nullable<Varchar>,
        workspace_id -> Int4,
    }
}

table! {
    containers (id) {
        id -> Int4,
        name -> Varchar,
        tenant_id -> Int4,
    }
}

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

table! {
    workspaces (id) {
        id -> Int4,
        display_order -> Int4,
        name -> Varchar,
        tenant_id -> Int4,
    }
}

joinable!(applets -> workspaces (workspace_id));
joinable!(containers -> tenants (tenant_id));
joinable!(workspaces -> tenants (tenant_id));

allow_tables_to_appear_in_same_query!(
    applets,
    containers,
    tenants,
    workspaces,
);
