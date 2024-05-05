// @generated automatically by Diesel CLI.

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
        project_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(repositories -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    repositories,
);
