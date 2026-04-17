use chrono::{NaiveDateTime, Utc};
use diesel::delete;
use diesel::insert_into;
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::containers::docker::DockerContainerListItem;
use crate::schema::containers;
use crate::schema::docker_host_containers;
use crate::tenants::error::MyError;


#[derive(Debug, Serialize, Deserialize, Identifiable, Clone, Queryable)]
#[table_name = "containers"]
pub struct Container {
    pub id: i32,
    pub name: String,
    #[serde(skip)]
    pub tenant_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "containers"]
pub struct NewContainer {
    pub name: String,
    pub tenant_id: i32,
}

impl Container {
    pub fn create(conn: &PgConnection, new_container: NewContainer) -> Result<Container, MyError> {
        return Ok(diesel::insert_into(containers::table)
            .values(NewContainer {
                name: new_container.name,
                tenant_id: new_container.tenant_id,
            })
            .get_result(conn)?);
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Container>> {
        return containers::table.load::<Container>(&*conn);
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct DockerHostContainer {
    pub docker_id: String,
    pub image: String,
    pub state: Option<String>,
    pub status: Option<String>,
    pub names: Value,
    pub compose_project: Option<String>,
    pub compose_service: Option<String>,
    pub labels: Option<Value>,
    pub synced_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "docker_host_containers"]
struct NewDockerHostContainerRow {
    docker_id: String,
    image: String,
    state: Option<String>,
    status: Option<String>,
    names: Value,
    compose_project: Option<String>,
    compose_service: Option<String>,
    labels: Option<Value>,
    synced_at: NaiveDateTime,
}

impl DockerHostContainer {
    pub fn replace_snapshot(conn: &PgConnection, items: &[DockerContainerListItem]) -> QueryResult<()> {
        conn.transaction(|| {
            delete(docker_host_containers::table).execute(conn)?;
            let now = Utc::now().naive_utc();
            let rows: Vec<NewDockerHostContainerRow> = items
                .iter()
                .filter(|c| !c.id.is_empty())
                .map(|c| NewDockerHostContainerRow {
                    docker_id: c.id.clone(),
                    image: c.image.clone(),
                    state: c.state.clone(),
                    status: c.status.clone(),
                    names: json!(c.names),
                    compose_project: c.compose_project.clone(),
                    compose_service: c.compose_service.clone(),
                    labels: c.labels.clone(),
                    synced_at: now,
                })
                .collect();
            if !rows.is_empty() {
                insert_into(docker_host_containers::table)
                    .values(&rows)
                    .execute(conn)?;
            }
            Ok(())
        })
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<DockerHostContainer>> {
        docker_host_containers::table.load::<DockerHostContainer>(conn)
    }

    pub fn to_list_item(&self) -> DockerContainerListItem {
        let names = serde_json::from_value(self.names.clone()).unwrap_or_default();
        DockerContainerListItem {
            id: self.docker_id.clone(),
            names,
            image: self.image.clone(),
            state: self.state.clone(),
            status: self.status.clone(),
            compose_project: self.compose_project.clone(),
            compose_service: self.compose_service.clone(),
            labels: self.labels.clone(),
        }
    }
}

