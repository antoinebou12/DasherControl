use crate::workspaces::handler::static_rocket_route_info_for_create_workspace;
use crate::workspaces::handler::static_rocket_route_info_for_create_applet;
use crate::workspaces::handler::static_rocket_route_info_for_get_applets_in_workspace;

pub fn create_workspaces_routes() -> Vec<rocket::Route> {
    return routes![
            create_workspace,
            create_applet,
            get_applets_in_workspace
        ];
}