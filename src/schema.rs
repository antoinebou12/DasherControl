table! {
    applets (id) {
        id -> Int4,
        name -> Varchar,
        position_x -> Int4,
        position_y -> Int4,
        width -> Int4,
        height -> Int4,
        editable -> Bool,
        applet_data -> Varchar,
        workspace_id -> Int4,
        is_floating -> Bool,
        float_left -> Nullable<Int4>,
        float_top -> Nullable<Int4>,
        float_width -> Nullable<Int4>,
        float_height -> Nullable<Int4>,
        float_z -> Nullable<Int4>,
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
    use diesel::sql_types::*;
    docker_host_containers (docker_id) {
        docker_id -> Varchar,
        image -> Text,
        state -> Nullable<Varchar>,
        status -> Nullable<Text>,
        names -> Jsonb,
        compose_project -> Nullable<Varchar>,
        compose_service -> Nullable<Varchar>,
        labels -> Nullable<Jsonb>,
        synced_at -> Timestamp,
    }
}

table! {
    login_history (id) {
        id -> Int4,
        tenant_id -> Int4,
        login_timestamp -> Timestamp,
    }
}

table! {
    tenant_configuration (id) {
        id -> Int4,
        tenant_id -> Int4,
        config -> Varchar,
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
        login_session -> Varchar,
        oauth_provider -> Nullable<Varchar>,
        oauth_subject -> Nullable<Varchar>,
    }
}

table! {
    workspaces (id) {
        id -> Int4,
        display_order -> Int4,
        name -> Varchar,
        tenant_id -> Int4,
        workspace_icon -> Varchar,
        theme_preset -> Varchar,
        bg_color -> Nullable<Varchar>,
        fg_color -> Nullable<Varchar>,
        accent_color -> Nullable<Varchar>,
        panel_color -> Nullable<Varchar>,
        background_image_url -> Nullable<Text>,
    }
}

joinable!(applets -> workspaces (workspace_id));
joinable!(containers -> tenants (tenant_id));
joinable!(login_history -> tenants (tenant_id));
joinable!(tenant_configuration -> tenants (tenant_id));
joinable!(workspaces -> tenants (tenant_id));

allow_tables_to_appear_in_same_query!(
    applets,
    containers,
    docker_host_containers,
    login_history,
    tenant_configuration,
    tenants,
    workspaces,
);
