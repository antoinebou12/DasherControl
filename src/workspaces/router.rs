use crate::workspaces::handler::static_rocket_route_info_for_create_workspace;
use crate::workspaces::handler::static_rocket_route_info_for_create_applet;
use crate::workspaces::handler::static_rocket_route_info_for_get_applets_in_workspace;
use crate::workspaces::handler::static_rocket_route_info_for_create_workspace_with_applets;
use crate::workspaces::handler::static_rocket_route_info_for_get_workspaces;
use crate::workspaces::handler::static_rocket_route_info_for_update_workspace;


pub fn create_workspaces_routes() -> Vec<rocket::Route> {
    return routes![
            create_workspace,
            create_applet,
            get_applets_in_workspace,
            create_workspace_with_applets,
            get_workspaces,
            update_workspace
        ];
}