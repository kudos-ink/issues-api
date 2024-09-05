// @generated automatically by Diesel CLI.

diesel::table! {
    issues (id) {
        id -> Int4,
        number -> Int4,
        #[max_length = 255]
        title -> Varchar,
        labels -> Nullable<Array<Nullable<Text>>>,
        open -> Bool,
        certified -> Nullable<Bool>,
        assignee_id -> Nullable<Int4>,
        repository_id -> Int4,
        issue_created_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        #[max_length = 255]
        slug -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
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
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 255]
        language_slug -> Varchar,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(issues -> repositories (repository_id));
diesel::joinable!(issues -> users (assignee_id));
diesel::joinable!(repositories -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    issues,
    languages,
    projects,
    repositories,
    users,
);
