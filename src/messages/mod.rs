pub mod common;

use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct NewProject {
    pub name: String,
    pub description: Option<String>,
}

pub struct NewProjectDTO {
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
}

impl NewProject {
    pub fn to_dto(&self, owner: &Uuid) -> NewProjectDTO {
        NewProjectDTO {
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            owner: *owner,
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewDocument {
    pub name: String,
    pub description: Option<String>,
    pub project_id: Uuid,
}

pub struct NewDocumentDTO {
    pub name: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub owner: Uuid,
}

impl NewDocument {
    pub fn to_dto(&self, owner: &Uuid) -> NewDocumentDTO {
        NewDocumentDTO {
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            project_id: self.project_id,
            owner: *owner,
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct CommonPropertiesUpdate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectId {
    pub project_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DocumentId {
    pub document_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DocumentElementId {
    pub document_id: Uuid,
    pub element_id: i32,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DocumentIdentifier {
    pub project_id: Uuid,
    pub document_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DomainAddEntityInput {
    pub name: String,
    pub project_id: Uuid,
    pub domain_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DocumentProperties {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectIdInput {
    pub project_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AttributeInputString {
    pub default: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AttributeInputInteger {
    pub default: Option<i32>,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AttributeInputBoolean {
    pub default: Option<bool>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AddStringAttributeToEntity {
    pub project_id: Uuid,
    pub domain_id: Uuid,
    pub entity_id: i32,
    pub name: String,
    pub default: Option<String>,
}

// FnGroup
#[derive(juniper::GraphQLInputObject)]
pub struct FnGroupFnNew {
    pub name: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct FnGroupFnUpdate {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
