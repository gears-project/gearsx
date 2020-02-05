use juniper;

use juniper::FieldResult;
use crate::db::models::{Document as DBDocument, Project as DBProject};
use crate::messages::*;
use crate::structure::domain::{Attribute, DomainDocument, Entity};
use crate::structure::modelx::ModelxDocument;
use super::schema::Context;

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn init_new_project(context: &Context, project: ProjectInput) -> FieldResult<DBProject> {
        debug!("init_new_project : {}", project.name);
        let mut conn = context.dbpool.get()?;
        Ok(DBProject::initialize_new_project(&conn, &project.name)?)
    }

    fn create_project(context: &Context, project: ProjectInput) -> FieldResult<DBProject> {
        debug!("create_project : {}", project.name);
        let mut conn = context.dbpool.get()?;
        Ok(DBProject::create(&conn, &project.name)?)
    }

    fn add_domain(context: &Context, domain: DomainInput) -> FieldResult<DomainDocument> {
        debug!("add_domain : {}", domain.name);
        let mut conn = context.dbpool.get()?;
        Ok(DBDocument::create_domain_document(
            &conn,
            &domain.project_id,
            &domain.name,
        )?)
    }

    fn add_model(context: &Context, input: ModelInput) -> FieldResult<ModelxDocument> {
        debug!("add_model for project {}", input.project_id);
        let mut conn = context.dbpool.get()?;
        Ok(DBDocument::create_model_document(&conn, &input)?)
    }

    fn domain_add_entity(context: &Context, input: DomainAddEntityInput) -> FieldResult<Entity> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DBDocument::by_id(&conn, &input.domain_id)?.as_domain()?;
        let entity = doc.body.add_entity(&input.name)?;
        let _ = DBDocument::save(&conn, &doc.as_raw());
        Ok(entity)
    }

    fn entity_add_string_attribute(
        context: &Context,
        input: AddStringAttributeToEntity,
    ) -> FieldResult<Attribute> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DBDocument::by_id(&conn, &input.domain_id)?.as_domain()?;
        let attribute = doc
            .body
            .entity_add_string_attribute(input.entity_id, &input)?;
        let _ = DBDocument::save(&conn, &doc.as_raw());
        Ok(attribute)
    }

    fn delete_project(context: &Context, input: ProjectIdInput) -> FieldResult<i32> {
        let mut conn = context.dbpool.get()?;
        let res = DBDocument::delete_project(&conn, &input.project_id)?;
        Ok(1)
    }
}
