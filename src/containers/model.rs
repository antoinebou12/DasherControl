use diesel::{PgConnection, QueryResult, RunQueryDsl, select};

use crate::schema::containers;
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
                tenant_id: 1
            })
            .get_result(conn)?);
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Container>> {
        return containers::table.load::<Container>(&*conn);
    }

}


