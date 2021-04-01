use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, select};

use crate::schema::applets;
use crate::schema::workspaces;
use crate::tenants::error::MyError;
use crate::schema::applets::workspace_id;


#[derive(Debug, Serialize, Deserialize, Identifiable, Clone, Queryable)]
#[table_name = "workspaces"]
pub struct Workspace {
    #[serde(skip)]
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
    #[serde(skip)]
    pub display_order: i32,
    pub name: String,
    #[serde(skip)]
    pub tenant_id: i32
}


impl Workspace {
    pub fn create(new_workspace: NewWorkspace, conn: &PgConnection) -> Result<Workspace, MyError> {
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

}


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

impl Applet {
    pub fn create(new_applet: NewApplet, conn: &PgConnection) -> Result<Applet, MyError> {
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

    pub fn all_applets_by_workspace(workspaces_id: i32, conn: &PgConnection) -> QueryResult<Vec<Applet>> {
        return applets::table.filter(workspace_id.eq(workspace_id)).load::<Applet>(&*conn);
    }
}
