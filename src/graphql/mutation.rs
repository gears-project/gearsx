use juniper;

use super::schema::Context;
use uuid::Uuid;
use crate::db::models::{Document as DocumentDAO, Project as ProjectDAO};
use crate::messages::*;
use crate::structure::domain::{Attribute, DomainDocument, Entity};
use crate::structure::xflow::{XFlowDocument};
use crate::structure::modelx::ModelxDocument;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn init_new_project(context: &Context, project: ProjectInput) -> FieldResult<ProjectDAO> {
        debug!("init_new_project : {}", project.name);
        let mut conn = context.dbpool.get()?;
        Ok(ProjectDAO::initialize_new_project(&conn, &project.name)?)
    }

    fn update_project(context: &Context, input: CommonPropertiesUpdate) -> FieldResult<ProjectDAO> {
        let mut conn = context.dbpool.get()?;
        Ok(ProjectDAO::update_project(&conn, input)?)
    }

    fn delete_project(context: &Context, input: ProjectIdInput) -> FieldResult<i32> {
        let mut conn = context.dbpool.get()?;
        let res = DocumentDAO::delete_project(&conn, &input.project_id)?;
        Ok(1)
    }

    fn add_xflow(context: &Context, doc: NewDocument) -> FieldResult<XFlowDocument> {
        debug!("add_xflow : {}", doc.name);
        let mut conn = context.dbpool.get()?;
        Ok(DocumentDAO::create_xflow_document(
            &conn,
            &doc.project_id,
            &doc.name,
        )?)
    }

    fn add_domain(context: &Context, doc: NewDocument) -> FieldResult<DomainDocument> {
        debug!("add_domain : {}", doc.name);
        let mut conn = context.dbpool.get()?;
        Ok(DocumentDAO::create_domain_document(
            &conn,
            &doc.project_id,
            &doc.name,
        )?)
    }

    fn delete_document(context: &Context, input: DocumentId) -> FieldResult<Uuid> {
        let mut conn = context.dbpool.get()?;
        let res = ProjectDAO::delete_document(&conn, &input.document_id)?;
        Ok(input.document_id)
    }

    fn add_model(context: &Context, input: ModelInput) -> FieldResult<ModelxDocument> {
        debug!("add_model for project {}", input.project_id);
        let mut conn = context.dbpool.get()?;
        Ok(DocumentDAO::create_model_document(&conn, &input)?)
    }

    fn domain_add_entity(context: &Context, input: DomainAddEntityInput) -> FieldResult<Entity> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &input.domain_id)?.as_domain()?;
        let entity = doc.body.add_entity(&input.name)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(entity)
    }

    fn entity_add_string_attribute(
        context: &Context,
        input: AddStringAttributeToEntity,
    ) -> FieldResult<Attribute> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &input.domain_id)?.as_domain()?;
        let attribute = doc
            .body
            .entity_add_string_attribute(input.entity_id, &input)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(attribute)
    }

    /*
    fn remove_entity(
        context: &Context,
        input: DocumentElementId,
    ) -> FieldResult<Attribute> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &input.document_id)?.as_domain()?;
        let attribute = doc
            .body
            .remove_entity(input.element_id)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(attribute)
    }
    */
}
