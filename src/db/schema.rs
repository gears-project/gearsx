table! {
    documents (id) {
        id -> Uuid,
        project_id -> Uuid,
        name -> Text,
        doctype -> Text,
        version -> Int4,
        body -> Jsonb,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        model_id -> Nullable<Uuid>,
    }
}

allow_tables_to_appear_in_same_query!(documents, projects,);
