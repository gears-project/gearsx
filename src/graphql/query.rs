use super::schema::Context;
use crate::db::models::{Document as DBDocument, Project as DBProject};
use crate::messages::*;
use crate::structure::domain::{Attribute, DomainDocument, Entity};
use crate::structure::xflow::{XFlowDocument};
use juniper;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all projects")]
    fn projects(context: &Context, paging: Option<QueryPage>) -> FieldResult<Vec<DBProject>> {
        let mut conn = context.dbpool.get()?;
        let projects = DBProject::find(&conn, paging)?;
        Ok(projects)
    }

    #[graphql(description = "Fetch a project by id")]
    fn project(context: &Context, input: ProjectIdInput) -> FieldResult<DBProject> {
        let mut conn = context.dbpool.get()?;
        Ok(DBProject::by_id(&conn, &input.project_id)?)
    }

    #[graphql(description = "List of all domain documents")]
    fn domains(context: &Context, input: ProjectId) -> FieldResult<Vec<DomainDocument>> {
        let mut conn = context.dbpool.get()?;
        let documents = DBDocument::find_domains(&conn, &input.project_id)?;
        Ok(documents)
    }

    #[graphql(description = "Fetch a domain document by id")]
    fn domain(context: &Context, input: DocumentId) -> FieldResult<DomainDocument> {
        let mut conn = context.dbpool.get()?;
        let doc = DBDocument::by_id(&conn, &input.document_id)?.as_domain()?;
        Ok(doc)
    }

    #[graphql(description = "List of all xflow documents")]
    fn xflows(context: &Context, input: ProjectId) -> FieldResult<Vec<XFlowDocument>> {
        let mut conn = context.dbpool.get()?;
        let documents = DBDocument::find_xflows(&conn, &input.project_id)?;
        Ok(documents)
    }

}
