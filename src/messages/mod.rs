use uuid::Uuid;
use crate::structure::data::{VTypeString, VTypeBoolean, VTypeInteger};

#[derive(juniper::GraphQLInputObject)]
pub struct QueryPage {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Default for QueryPage {
    fn default() -> Self {
        Self {
            limit: None,
            offset: Some(0),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ModelInput {
    pub name: String,
    pub description: Option<String>,
    pub project_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DomainInput {
    pub name: String,
    pub description: Option<String>,
    pub project_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct DomainIdentifier {
    pub project_id: Uuid,
    pub domain_id: Uuid,
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
    // pub vtype: VTypeString,
}
