use juniper;
use juniper::FieldResult;
use crate::messages::*;
use crate::db::models::{Document as DBDocument, Project as DBProject};
use crate::structure::domain::{Attribute, DomainDocument, Entity};
use super::schema::Context;

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all projects")]
    fn projects(context: &Context, paging: Option<QueryPage>) -> FieldResult<Vec<DBProject>> {
        let mut conn = context.dbpool.get()?;
        let projects = DBProject::find(&conn, paging)?;
        Ok(projects)
    }

    #[graphql(description = "List of all domain documents")]
    fn domains(context: &Context, project: ProjectIdInput) -> FieldResult<Vec<DomainDocument>> {
        let mut conn = context.dbpool.get()?;
        let documents = DBDocument::find_domains(&conn, &project.project_id)?;
        Ok(documents)
    }
}
