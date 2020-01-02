use juniper;
use juniper::{FieldError, FieldResult, RootNode};

use crate::db::connection::Pool;
use crate::db::models::Project;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all projects")]
    fn projects(context: &Context) -> FieldResult<Vec<Project>> {
        let mut conn = context.dbpool.get().unwrap();
        let projects = Project::find(&conn).unwrap();
        Ok(projects)
    }
}

pub struct MutationRoot;

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectInput {
    name: String
}

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_project(context: &Context, project: ProjectInput) -> FieldResult<Project> {
        let mut conn = context.dbpool.get().unwrap();
        Ok(Project::create(&project.name, &conn).unwrap())
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}

