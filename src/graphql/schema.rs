use juniper::RootNode;

use crate::db::connection::Pool;

use super::query::QueryRoot;
use super::mutation::MutationRoot;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Self {
            dbpool: crate::db::connection::get_connection_pool()
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
