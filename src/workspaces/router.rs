use crate::workspaces::handler;

pub fn create_workspaces_routes() -> Vec<rocket::Route> {
    routes![
        handler::create_workspace,
        handler::create_applet,
        handler::get_applets_in_workspace,
        handler::create_workspace_with_applets,
        handler::get_workspaces,
        handler::update_workspace,
        handler::delete_workspace_with_applets,
    ]
}
