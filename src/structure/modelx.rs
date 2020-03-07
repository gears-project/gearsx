use super::common::Document;
use super::domain::DomainDocument;
use super::xflow::XFlowDocument;
use crate::db::models::Document as DBDocument;
use crate::graphql::schema;
use uuid::Uuid;

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

    fn xflows(&self, context: &schema::Context) -> juniper::FieldResult<Vec<XFlowDocument>> {
        let mut conn = context.dbpool.get().unwrap();
        let xflows = DBDocument::find_xflows(&conn, &self.project_id).unwrap();
        Ok(xflows)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Modelx {}

#[juniper::object]
impl Modelx {
    fn id(&self) -> i32 {
        1
    }
}

impl Default for Modelx {
    fn default() -> Self {
        Self {}
    }
}
