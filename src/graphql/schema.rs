use uuid::Uuid;
use juniper;
use juniper::{FieldResult, RootNode};

use crate::db::connection::Pool;
use crate::db::models::{Project as DBProject, Document as DBDocument};
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

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_project(context: &Context, project: ProjectInput) -> FieldResult<DBProject> {
        let mut conn = context.dbpool.get().unwrap();
        Ok(DBProject::create(&project.name, &conn).unwrap())
    }

    fn add_domain(context: &Context, domain: DomainInput) -> FieldResult<DomainDocument> {
        let mut conn = context.dbpool.get().unwrap();
        Ok(DBDocument::create(&domain.project_id, &domain.name, &conn).unwrap().as_domain().unwrap())
    }
}


