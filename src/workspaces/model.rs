use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, deserialize, serialize, update};

use crate::schema::applets;
use crate::schema::workspaces;
use crate::tenants::error::MyError;
use crate::schema::applets::dsl::{workspace_id, id as aid};
use crate::schema::workspaces::dsl::{tenant_id, id as wid, display_order, name};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use serde_json::Value;
use diesel::sql_types::{Varchar};
use diesel::serialize::{ToSql, Output};
use std::io::Write;
use diesel::pg::Pg;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset)]
#[table_name = "applets"]
pub struct Applet {
    pub id: i32,
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub editable: bool,
    pub applet_data: DBJsonType,
    pub workspace_id: i32
}

#[derive(Debug, Serialize, Deserialize, Insertable, PartialEq)]
#[table_name = "applets"]
pub struct NewApplet {
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub editable: bool,
    pub applet_data: DBJsonType,
    pub workspace_id: i32
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NewAppletNoWorkspace {
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub editable: bool,
    pub applet_data: DBJsonType
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
        return applets::table.load::<Applet>(conn);
    }

    pub fn all_applets_by_workspace(conn: &PgConnection, id: i32) -> QueryResult<Vec<Applet>> {
        return applets::table.filter(workspace_id.eq(&id)).order(aid.asc()).load::<Applet>(conn);
    }

    pub fn delete_all_from_workspace(conn: &PgConnection, id: i32) -> QueryResult<usize> {
        return diesel::delete(applets::table.filter(workspace_id.eq(&id))).execute(conn);
    }

    pub fn update(conn: &PgConnection, change_applet: Applet) -> Result<Applet, MyError> {
        return Ok(diesel::update(applets::table.filter(aid.eq(change_applet.id)))
            .set(Applet {
                id: change_applet.id,
                name: change_applet.name,
                position_x: change_applet.position_x,
                position_y: change_applet.position_y,
                width: change_applet.width,
                height: change_applet.height,
                editable: change_applet.editable,
                applet_data: change_applet.applet_data,
                workspace_id: change_applet.workspace_id
            })
            .get_result(conn)?);
    }
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Clone, Queryable)]
#[table_name = "workspaces"]
pub struct Workspace {
    pub id: i32,
    pub display_order: i32,
    pub name: String,
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
pub struct WorkspaceWithApplets {
    pub id: i32,
    pub display_order: i32,
    pub name: String,
    pub tenant_id: i32,
    pub applets: Vec<NewAppletNoWorkspace>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorkspaceWithApplets {
    pub display_order: i32,
    pub name: String,
    pub tenant_id: i32,
    pub applets: Vec<NewAppletNoWorkspace>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceNoIDWithApplets {
    pub display_order: i32,
    pub name: String,
    pub applets: Vec<NewAppletNoWorkspace>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceNoTenantWithApplets {
    pub id: i32,
    pub display_order: i32,
    pub name: String,
    pub applets: Vec<NewAppletNoWorkspace>
}



impl Workspace {
    pub fn create(conn: &PgConnection, new_workspace: NewWorkspace) -> Result<Workspace, MyError> {
        return Ok(diesel::insert_into(workspaces::table)
            .values(NewWorkspace {
                display_order: new_workspace.display_order,
                name: new_workspace.name,
                tenant_id: new_workspace.tenant_id
            })
            .get_result(conn)?);
    }

    pub fn get(conn: &PgConnection, id: &i32) -> Result<Workspace, diesel::result::Error> {
        return workspaces::table.filter(wid.eq(id)).first::<Workspace>(conn);
    }

    pub fn update(conn: &PgConnection, update_workspace: Workspace) -> Result<Workspace, MyError> {
        return Ok(diesel::update(
            workspaces::table.filter(wid.eq(update_workspace.id)))
            .set(
                (
                    display_order.eq(update_workspace.display_order),
                    name.eq(update_workspace.name),
                ))

            .get_result(conn)?);
    }

    pub fn delete_with_applets(conn: &PgConnection, id: &i32) -> usize {
        diesel::delete(applets::table.filter(workspace_id.eq(id))).execute(conn).unwrap();
        return diesel::delete(workspaces::table.filter(wid.eq(id))).execute(conn).unwrap();
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Workspace>> {
        return workspaces::table.load::<Workspace>(conn);
    }

    pub fn all_for_tenant(conn: &PgConnection, id: &i32) -> Result<Vec<Workspace>, diesel::result::Error> {
        return workspaces::table
            .filter(tenant_id.eq(&id))
            .order(display_order.asc())
            .load::<Workspace>(conn);
    }

    pub fn create_with_applets(conn: &PgConnection, new_workspace: NewWorkspaceWithApplets)
        -> Result<Workspace, MyError> {
        let add_workspace = NewWorkspace {
            display_order: new_workspace.display_order,
            name: new_workspace.name,
            tenant_id: new_workspace.tenant_id
        };
        let workspace = Workspace::create(conn, add_workspace)?;
        for new_applet in new_workspace.applets {
            let new_applet_workspace_id = NewApplet {
                name: new_applet.name,
                position_x: new_applet.position_x,
                position_y: new_applet.position_y,
                width: new_applet.width,
                height: new_applet.height,
                editable: new_applet.editable,
                applet_data: new_applet.applet_data,
                workspace_id: workspace.id
            };
            let _applets = Applet::create(conn, new_applet_workspace_id);
        }
        return Ok(workspace)
    }

    pub fn update_with_applets(conn: &PgConnection, update_workspace: WorkspaceWithApplets)
                               -> Result<Workspace, MyError> {
        let change_workspace = Workspace {
            id: update_workspace.id,
            display_order: update_workspace.display_order,
            name: update_workspace.name,
            tenant_id: update_workspace.tenant_id
        };
        let workspace = Workspace::update(conn, change_workspace)?;
        let mut current_applets = Applet::all_applets_by_workspace(conn, update_workspace.id).unwrap();
        let mut count = 0;
        if update_workspace.applets.len() == 0 {
            Applet::delete_all_from_workspace(conn, update_workspace.id);
        }
        for change_applet in update_workspace.applets {
            if count >= current_applets.len() {
                let new_applet_workspace_id = NewApplet {
                    name: change_applet.name,
                    position_x: change_applet.position_x,
                    position_y: change_applet.position_y,
                    width: change_applet.width,
                    height: change_applet.height,
                    editable: change_applet.editable,
                    applet_data: change_applet.applet_data,
                    workspace_id: workspace.id
                };
                let _applets = Applet::create(conn, new_applet_workspace_id);
            } else {
                let current_applet = &mut current_applets[count];
                    let change_applet_workspace_id = Applet {
                        id: current_applet.id,
                        name: change_applet.name,
                        position_x: change_applet.position_x,
                        position_y: change_applet.position_y,
                        width: change_applet.width,
                        height: change_applet.height,
                        editable: change_applet.editable,
                        applet_data: change_applet.applet_data,
                        workspace_id: update_workspace.id
                    };
                    let _applets = Applet::update(conn, change_applet_workspace_id);
            }
            count += 1;
        }
        return Ok(workspace)
    }

}


// Custom json as a varchar
impl FromSql<Varchar, Pg> for DBJsonType {
    fn from_sql(
        bytes: Option<&<Pg as Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let t = <String as FromSql<Varchar, Pg>>::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&t)?))
    }
}

impl ToSql<Varchar, Pg> for DBJsonType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        <String as ToSql<Varchar, Pg>>::to_sql(&s, out)
    }
}

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, PartialEq, Clone)]
#[sql_type = "Varchar"]
pub struct DBJsonType(Value);