// @generated automatically by Diesel CLI.

diesel::table! {
    languages (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        categories -> Nullable<Array<Nullable<Text>>>,
        purposes -> Nullable<Array<Nullable<Text>>>,
        stack_levels -> Nullable<Array<Nullable<Text>>>,
        technologies -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    repositories (id) {
        id -> Int4,
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        language_id -> Int4,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(repositories -> languages (language_id));
diesel::joinable!(repositories -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    languages,
    projects,
    repositories,
    users,
);
