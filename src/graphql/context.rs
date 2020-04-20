use uuid::Uuid;
use crate::db::connection::Pool;

pub struct Context {
    pub dbpool: Pool,
    pub user: Uuid,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Self {
            dbpool: crate::db::connection::get_connection_pool(),
            user: crate::util::naming::empty_uuid(),
        }
    }
}

