use juniper::RootNode;

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
