use juniper;
use juniper::{FieldResult, RootNode};
use uuid::Uuid;

use crate::db::connection::Pool;
use crate::db::models::{Document as DBDocument, Project as DBProject};
use crate::structure::domain::DomainDocument;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all projects")]
    fn projects(context: &Context) -> FieldResult<Vec<DBProject>> {
        let mut conn = context.dbpool.get().unwrap();
        let projects = DBProject::find(&conn).unwrap();
        Ok(projects)
    }

    #[graphql(description = "List of all domain documents")]
    fn domains(context: &Context) -> FieldResult<Vec<DomainDocument>> {
        let mut conn = context.dbpool.get().unwrap();
        let documents = DBDocument::find_domains(&conn).unwrap();
        Ok(documents)
    }
}

pub struct MutationRoot;

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectInput {
    name: String,
    description: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DomainInput {
    name: String,
    description: Option<String>,
    project_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DomainAddEntityInput {
    name: String,
    project_id: Uuid,
    domain_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectIdInput {
    project_id: Uuid,
}

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_project(context: &Context, project: ProjectInput) -> FieldResult<DBProject> {
        let mut conn = context.dbpool.get()?;
        Ok(DBProject::create(&conn, &project.name)?)
    }

    fn add_domain(context: &Context, domain: DomainInput) -> FieldResult<DomainDocument> {
        let mut conn = context.dbpool.get()?;
        Ok(DBDocument::create_domain_document(
            &conn,
            &domain.project_id,
            &domain.name,
        )?)
    }

    fn domain_add_entity(
        context: &Context,
        input: DomainAddEntityInput,
    ) -> FieldResult<DomainDocument> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DBDocument::by_id(&conn, &input.domain_id)?.as_domain()?;
        let _ = doc.body.add_entity(&input.name)?;
        let _ = DBDocument::save(&conn, &doc.as_raw());
        Ok(doc)
    }

    fn delete_project(context: &Context, input: ProjectIdInput) -> FieldResult<i32> {
        let mut conn = context.dbpool.get()?;
        let res = DBDocument::delete_project(&conn, &input.project_id)?;
        Ok(1)
    }
}
