use super::schema::{projects, documents};
use uuid::Uuid;
use serde_json;

#[derive(Insertable, Queryable)]
#[table_name="projects"]
pub struct Project<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Insertable, Queryable)]
#[table_name="documents"]
pub struct Document<'a> {
    pub id: &'a Uuid,
    pub project_id: &'a Uuid,
    pub name: &'a str,
    pub doctype: &'a str,
    pub body: &'a serde_json::Value,
}
