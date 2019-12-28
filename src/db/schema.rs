table! {
    documents (id) {
        id -> Uuid,
        project_id -> Uuid,
        name -> Text,
        doctype -> Text,
        body -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

joinable!(documents -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    documents,
    projects,
);
