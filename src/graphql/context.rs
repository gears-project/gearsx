use crate::db::connection::Pool;
use uuid::Uuid;

pub struct Context {
    pub dbpool: Pool,
    pub user: Uuid,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(uuid: &Uuid) -> Self {
        Self {
            dbpool: crate::db::connection::get_connection_pool(),
            user: *uuid,
        }
    }
}
