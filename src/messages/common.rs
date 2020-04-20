
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
