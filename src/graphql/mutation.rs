use juniper;

use super::context::Context;
use uuid::Uuid;
use crate::db::models::{Document as DocumentDAO, Project as ProjectDAO};
use crate::messages::*;
use crate::structure::domain::{Attribute, DomainDocument, Entity};
use crate::structure::xflow::{XFlowDocument};
use crate::structure::fngroup::{FngroupDocument, FnDefinition};
use crate::structure::modelx::ModelxDocument;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn init_new_project(context: &Context, project: NewProject) -> FieldResult<ProjectDAO> {
        debug!("init_new_project : {}", project.name);
        let mut conn = context.dbpool.get()?;
        Ok(ProjectDAO::initialize_new_project(&conn, &project.to_dto(&context.user))?)
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
            &doc.to_dto(&context.user)
        )?)
    }

    fn add_domain(context: &Context, doc: NewDocument) -> FieldResult<DomainDocument> {
        debug!("add_domain : {}", doc.name);
        let mut conn = context.dbpool.get()?;
        Ok(DocumentDAO::create_domain_document(
            &conn,
            &doc.to_dto(&context.user)
        )?)
    }

    fn add_fngroup(context: &Context, doc: NewDocument) -> FieldResult<FngroupDocument> {
        debug!("add_fngroup : {}", doc.name);
        let mut conn = context.dbpool.get()?;
        Ok(DocumentDAO::create_fngroup_document(
            &conn,
            &doc.to_dto(&context.user)
        )?)
    }

    fn delete_document(context: &Context, input: DocumentId) -> FieldResult<Uuid> {
        let mut conn = context.dbpool.get()?;
        let res = ProjectDAO::delete_document(&conn, &input.document_id)?;
        Ok(input.document_id)
    }

    fn add_model(context: &Context, doc: NewDocument) -> FieldResult<ModelxDocument> {
        debug!("add_model for project {}", doc.project_id);
        let mut conn = context.dbpool.get()?;
        Ok(DocumentDAO::create_model_document(&conn, &doc.to_dto(&context.user))?)
    }

    fn domain_add_entity(context: &Context, input: DomainAddEntityInput) -> FieldResult<Entity> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &input.domain_id)?.as_domain()?;
        let entity = doc.body.add_entity(&input.name)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(entity)
    }

    /*
    fn fngroup_add_fn(context: &Context, input: FnGroupAPIAddFn) -> FieldResult<FnDefinition> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &input.domain_id)?.as_domain()?;
        let entity = doc.body.add_entity(&input.name)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(entity)
    }
    */

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

    fn fngroup_add_fn(context: &Context, doc: DocumentIdentifier, input: FnGroupFnNew) -> FieldResult<FnDefinition> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &doc.document_id)?.as_fngroup()?;
        let obj = doc.body.add_fn(&input.name)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(obj)
    }

    fn fngroup_update_fn(
        context: &Context,
        doc: DocumentIdentifier,
        input: FnGroupFnUpdate,
    ) -> FieldResult<FnDefinition> {
        let mut conn = context.dbpool.get()?;
        let mut doc = DocumentDAO::by_id(&conn, &doc.document_id)?.as_fngroup()?;
        let f = doc
            .body
            .update_fn(&input)?;
        let _ = DocumentDAO::save(&conn, &doc.as_raw());
        Ok(f)
    }

}
