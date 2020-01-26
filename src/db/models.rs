use super::schema::{documents, projects};
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::structure::common::{RawDocument, DocumentReference};
use crate::structure::modelx::{Modelx, ModelxDocument};
use crate::structure::domain::{Domain, DomainDocument};
use crate::messages::{QueryPage};
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use serde_json;
use uuid::Uuid;
use crate::graphql::schema;

#[derive(
    Serialize, Deserialize, Debug, AsChangeset, Queryable, Insertable, Identifiable,
)]
#[table_name = "projects"]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub model_id: Option<Uuid>,
}

#[juniper::object(Context = schema::Context)]
impl Project {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn model(&self, context: &schema::Context) -> juniper::FieldResult<Option<ModelxDocument>> {
        let mut conn = context.dbpool.get()?;
        if let Some(id) = &self.model_id {
            Ok(Some(Document::by_id(&conn, id)?.as_modelx()?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, AsChangeset, Queryable, Insertable, Identifiable)]
#[table_name = "documents"]
pub struct Document {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub body: serde_json::Value,
}

impl Project {
    pub fn initialize_new_project(conn: &PgConnection, name: &str) -> Result<Project, DieselError> {
        debug!("initialize_new_project : {}", name);

        let mut project = Self::create(conn, &name)?;

        debug!("initialize_new_project : {} : project id : {}", name, project.id);

        let mut model = Document::create_model_document(conn, &crate::messages::ModelInput {
            name: name.to_string(),
            description: None,
            project_id: project.id,
        })?;

        debug!("initialize_new_project : {} : model id : {}", name, model.id);

        project.model_id = model.id.into();
        let updated_project = diesel::update(projects::table)
            .set(&project)
            .get_result::<Project>(conn)?;

        let domain = Document::create_domain_document(conn, &project.id, "Domain".into())?;
        debug!("initialize_new_project : {} : domain id : {}", name, domain.id);

        model.body.domain = Some(DocumentReference {
            id: domain.id,
            doctype: "domain".to_string(),
        });
        let _ = Document::save(conn, &model.as_raw())?;
        Ok(updated_project)

    }

    pub fn create(conn: &PgConnection, name: &str) -> Result<Project, DieselError> {
        let id = uuid::Uuid::new_v4();

        let project = Project {
            id: id,
            name: name.to_owned(),
            description: name.to_owned(),
            model_id: None,
        };

        diesel::insert_into(projects::table)
            .values(project)
            .get_result(conn)
    }

    pub fn by_id(conn: &PgConnection, id: &Uuid) -> Result<Project, DieselError> {
        projects::table.find(id).first::<Project>(conn)
    }

    pub fn find(conn: &PgConnection, paging: Option<QueryPage>) -> Result<Vec<Project>, DieselError> {
        let p = paging.unwrap_or(QueryPage::default());
        if let Some(limit) = p.limit {
            projects::table.limit(limit.into()).offset(p.offset.unwrap_or(0).into()).load::<Project>(conn)
        } else {
            projects::table.load::<Project>(conn)
        }
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
    Domain(DomainDocument),
}

impl Document {
    pub fn as_domain(&self) -> Result<DomainDocument, String> {
        debug!("as_domain {}", &self.doctype.as_str());
        match &self.doctype.as_str() {
            &"domain" => Ok(DomainDocument {
                id: self.id,
                project_id: self.project_id,
                doctype: self.doctype.clone(),
                name: self.name.clone(),
                version: self.version.clone(),
                body: serde_json::from_value::<Domain>(self.body.clone()).unwrap(),
            }),
            _ => Err("Not a domain document".to_owned()),
        }
    }

    pub fn as_modelx(&self) -> Result<ModelxDocument, String> {
        match &self.doctype.as_str() {
            &"modelx" => Ok(ModelxDocument {
                id: self.id,
                project_id: self.project_id,
                doctype: self.doctype.clone(),
                name: self.name.clone(),
                version: self.version.clone(),
                body: serde_json::from_value::<Modelx>(self.body.clone()).unwrap(),
            }),
            _ => Err("Not a modelx document".to_owned()),
        }
    }

    pub fn concrete(&self) -> Option<GearsDocument> {
        match &self.doctype.as_str() {
            &"domain" => Some(GearsDocument::Domain(self.as_domain().unwrap())),
            _ => None,
        }
    }

    fn from_raw(doc: &RawDocument) -> Self {
        Self {
            id: doc.id.to_owned(),
            project_id: doc.project_id.to_owned(),
            version: doc.version.to_owned(),
            name: doc.name.to_owned(),
            doctype: doc.doctype.to_owned(),
            body: doc.body.to_owned(),
        }
    }

    pub fn create_domain_document(
        conn: &PgConnection,
        project_id: &Uuid,
        name: &str,
    ) -> Result<DomainDocument, DieselError> {
        let mut doc = DomainDocument::new(&project_id, "domain".to_owned());
        doc.name = name.to_owned();
        let record = Self::from_raw(&doc.as_raw());

        let res: Self = diesel::insert_into(documents::table)
            .values(record)
            .get_result(conn)?;
        Ok(res.as_domain().unwrap())
    }

    pub fn create_model_document(
        conn: &PgConnection,
        input: &crate::messages::ModelInput,
    ) -> Result<ModelxDocument, DieselError> {
        let mut doc = ModelxDocument::new(&input.project_id, "modelx".to_owned());
        doc.name = input.name.to_owned();
        let record = Self::from_raw(&doc.as_raw());

        let res: Self = diesel::insert_into(documents::table)
            .values(record)
            .get_result(conn)?;
        Ok(res.as_modelx().unwrap())
    }

    pub fn save(conn: &PgConnection, doc: &RawDocument) -> Result<Self, DieselError> {
        let record = Self::from_raw(&doc);
        diesel::update(documents::table)
            .filter(documents::id.eq(doc.id))
            .set(&record)
            .get_result(conn)
    }

    pub fn by_id(conn: &PgConnection, id: &Uuid) -> Result<Document, DieselError> {
        documents::table.find(id).first::<Document>(conn)
    }

    pub fn find_domains(conn: &PgConnection) -> Result<Vec<DomainDocument>, DieselError> {
        Ok(documents::table
            .filter(documents::doctype.eq(&"domain"))
            .load::<Document>(conn)?
            .iter()
            .map(|res| res.as_domain().unwrap())
            .collect())
    }

    pub fn delete_project(conn: &PgConnection, id: &Uuid) -> Result<usize, DieselError> {
        diesel::delete(projects::table.find(id)).execute(conn)
    }
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
