use super::common::{Document, DocumentReference};
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
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Modelx {
    pub domain: Option<DocumentReference>,
}

#[juniper::object(Context = schema::Context)]
impl Modelx {
    fn domain(&self, context: &schema::Context) -> juniper::FieldResult<Option<DomainDocument>> {
        let mut conn = context.dbpool.get().unwrap();
        if let Some(domain) = &self.domain {
            let domain = DBDocument::by_id(&conn, &domain.id).unwrap().as_domain().unwrap();
            Ok(Some(domain))
        } else {
            Ok(None)
        }
    }
}

impl Default for Modelx {
    fn default() -> Self {
        Self {
            domain: None,
        }
    }
}
