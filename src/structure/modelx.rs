use super::common::{Document};
use super::domain::{DomainDocument};
use uuid::Uuid;
use crate::graphql::schema;
use crate::db::models::{Document as DBDocument};

pub type ModelxDocument = Document<Modelx>;

#[juniper::object(Context = schema::Context)]
impl ModelxDocument {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn doctype(&self) -> &str {
        &self.doctype
    }

    fn body(&self) -> &Modelx {
        &self.body
    }
    fn domains(&self, context: &schema::Context) -> juniper::FieldResult<Vec<DomainDocument>> {
        let mut conn = context.dbpool.get().unwrap();
        let domains = DBDocument::find_domains(&conn, &self.project_id).unwrap();
        Ok(domains)
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Modelx {
}

impl Default for Modelx {
    fn default() -> Self {
        Self {
        }
    }
}
