use super::schema::{projects, documents};
use crate::structure::domain::{Domain, DomainDocument};
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
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
    pub version: i32,
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

pub enum GearsDocument {
    Domain(DomainDocument)
}

impl Document {

    pub fn as_domain(&self) -> Result<DomainDocument, String> {
        match &self.doctype.as_str() {
            &"domain" => {
                Ok(DomainDocument {
                    id: self.id,
                    doctype: self.doctype.clone(),
                    name: self.name.clone(),
                    version: self.version.clone(),
                    body: serde_json::from_value::<Domain>(self.body.clone()).unwrap()
                })
            },
            _ => Err("Not a domain document".to_owned()),
        }
    }

    pub fn concrete(&self) -> Option<GearsDocument> {
        match &self.doctype.as_str() {
            &"domain" => {
                Some(GearsDocument::Domain(self.as_domain().unwrap()))
            },
            _ => None,
        }
    }

    pub fn create(project_id: &Uuid, name: &str, conn: &PgConnection) -> Result<Self, DieselError> {

        let doc = DomainDocument::default();

        let record = Self {
            id: doc.id,
            project_id: project_id.to_owned(),
            version: 0,
            name: name.to_owned(),
            doctype: "domain".to_owned(),
            body: serde_json::to_value(doc.body).unwrap(),
        };

        diesel::insert_into(documents::table)
            .values(record)
	    .get_result(conn)

    }

    pub fn by_id(id: &Uuid, conn: &PgConnection) -> Result<Document, DieselError> {
        documents::table.find(id)
            .first::<Document>(conn)
    }

    pub fn find_domains(conn: &PgConnection) -> Result<Vec<DomainDocument>, DieselError> {
        Ok(documents::table
            .filter(documents::doctype.eq(&"domain"))
            .load::<Document>(conn)
            .unwrap()
            .iter()
            .map(|res| {
                res.as_domain().unwrap()
            })
            .collect()
        )
            
    }

/*
    pub fn delete(id: &str, connection: &PgConnection) -> Result<(), DieselError> {
        diesel::delete(projects::table.find(id))
            .execute(connection)?;
        Ok(())
    }
*/
}

/*
juniper::graphql_union!(<'a> &'a Docco: () as "Document" where Scalar = <S> |&self| { 
    instance_resolvers: |_| {
        // The left hand side indicates the concrete type T, the right hand
        // side should be an expression returning Option<T>
        &DomainDocument => self.as_domain(),
    }
});
*/

