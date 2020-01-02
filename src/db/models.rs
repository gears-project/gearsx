use super::schema::{projects, documents};
use crate::structure::domain::DomainDocument;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use uuid::Uuid;
use serde_json;

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(AsChangeset, Queryable, Insertable)]
#[table_name="projects"]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(AsChangeset, Queryable, Insertable)]
#[table_name="documents"]
pub struct Document {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub doctype: String,
    pub body: serde_json::Value,
}

impl Project {
    pub fn create(name: &str, conn: &PgConnection) -> Result<Project, DieselError> {

	let id = uuid::Uuid::new_v4();

	let project = Project {
	    id: id,
	    name: name.to_owned(),
	    description: name.to_owned(),
	};

        diesel::insert_into(projects::table)
            .values(project)
	    .get_result(conn)
    }

    pub fn by_id(id: &Uuid, conn: &PgConnection) -> Result<Project, DieselError> {
        projects::table.find(id)
            .first::<Project>(conn)
    }

    pub fn find(conn: &PgConnection) -> Result<Vec<Project>, DieselError> {
        projects::table
            .load::<Project>(conn)
    }


/*
    pub fn delete(id: &str, connection: &PgConnection) -> Result<(), DieselError> {
        diesel::delete(projects::table.find(id))
            .execute(connection)?;
        Ok(())
    }
*/
}

impl Document {
    pub fn create(project_id: &Uuid, name: &str, conn: &PgConnection) -> Result<Self, DieselError> {

        let doc = DomainDocument::default();

	let record = Self {
	    id: doc.id,
	    project_id: project_id.to_owned(),
	    name: name.to_owned(),
            doctype: "domain".to_owned(),
            body: serde_json::to_value(doc.body).unwrap(),
	};

        diesel::insert_into(documents::table)
            .values(record)
	    .get_result(conn)

    }

    pub fn by_id(id: &Uuid, conn: &PgConnection) -> Result<Project, DieselError> {
        projects::table.find(id)
            .first::<Project>(conn)
    }

/*
    pub fn delete(id: &str, connection: &PgConnection) -> Result<(), DieselError> {
        diesel::delete(projects::table.find(id))
            .execute(connection)?;
        Ok(())
    }
*/
}

