use uuid::Uuid;

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
pub struct DomainAddEntityInput {
    pub name: String,
    pub project_id: Uuid,
    pub domain_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProjectIdInput {
    pub project_id: Uuid,
}
