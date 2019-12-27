table! {
    documents (id) {
        id -> Uuid,
        project_id -> Uuid,
        doctype -> Text,
        version -> Int4,
        doc -> Jsonb,
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
