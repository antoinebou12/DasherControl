use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, select};

use crate::schema::applets;
use crate::schema::workspaces;
use crate::tenants::error::MyError;
use crate::schema::applets::workspace_id;



#[derive(Debug, Serialize, Deserialize, Identifiable, Clone, Queryable)]
#[table_name = "applets"]
pub struct Applet {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub editable: bool,
    pub applet_data: Option<String>,
    #[serde(skip)]
    pub workspace_id: i32
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "applets"]
pub struct NewApplet {
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub editable: bool,
    pub applet_data: Option<String>,
    pub workspace_id: i32
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAppletNoWorkspace {
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub editable: bool,
    pub applet_data: Option<String>,
}

impl Applet {
    pub fn create(conn: &PgConnection, new_applet: NewApplet) -> Result<Applet, MyError> {
        return Ok(diesel::insert_into(applets::table)
            .values(NewApplet {
                name: new_applet.name,
                position_x: new_applet.position_x,
                position_y: new_applet.position_y,
                width: new_applet.width,
                height: new_applet.height,
                editable: new_applet.editable,
                applet_data: new_applet.applet_data,
                workspace_id: new_applet.workspace_id
            })
            .get_result(conn)?);
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Applet>> {
        return applets::table.load::<Applet>(&*conn);
    }

    pub fn all_applets_by_workspace(conn: &PgConnection, id: i32) -> QueryResult<Vec<Applet>> {
        return applets::table.filter(workspace_id.eq(&id)).load::<Applet>(&*conn);
    }
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Clone, Queryable)]
#[table_name = "workspaces"]
pub struct Workspace {
    pub id: i32,
    #[serde(skip)]
    pub display_order: i32,
    pub name: String,
    #[serde(skip)]
    pub tenant_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "workspaces"]
pub struct NewWorkspace {
    pub display_order: i32,
    pub name: String,
    pub tenant_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorkspaceWithApplets {
    pub display_order: i32,
    pub name: String,
    pub tenant_id: i32,
    pub applets: Vec<NewAppletNoWorkspace>
}


impl Workspace {
    pub fn create(conn: &PgConnection, new_workspace: NewWorkspace) -> Result<Workspace, MyError> {
        return Ok(diesel::insert_into(workspaces::table)
            .values(NewWorkspace {
                display_order: 0,
                name: new_workspace.name,
                tenant_id: 1
            })
            .get_result(conn)?);
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Workspace>> {
        return workspaces::table.load::<Workspace>(&*conn);
    }

    pub fn create_with_applets(conn: &PgConnection, new_workspace: NewWorkspaceWithApplets)
        -> Result<Workspace, MyError> {
        let add_workspace = NewWorkspace {
            display_order: new_workspace.display_order,
            name: new_workspace.name,
            tenant_id: new_workspace.tenant_id
        };
        let workspace = Workspace::create(&*conn, add_workspace)?;
        for new_applet in new_workspace.applets {
            let new_applet_worksapce_id = NewApplet {
                name: new_applet.name,
                position_x: new_applet.position_x,
                position_y: new_applet.position_y,
                width: new_applet.width,
                height: new_applet.height,
                editable: new_applet.editable,
                applet_data: new_applet.applet_data,
                workspace_id: workspace.id
            };
            let applets = Applet::create(&*conn, new_applet_worksapce_id);
        }
        return Ok(workspace)
    }

}


