table! {
    documents (id) {
        id -> Uuid,
        project_id -> Uuid,
        name -> Text,
        doctype -> Text,
        version -> Int4,
        body -> Jsonb,
        owner -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        model_id -> Nullable<Uuid>,
        owner -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    documents,
    projects,
);
