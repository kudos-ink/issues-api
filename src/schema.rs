// @generated automatically by Diesel CLI.

diesel::table! {
    issues (id) {
        id -> Int4,
        number -> Int4,
        title -> Text,
        labels -> Nullable<Array<Nullable<Text>>>,
        open -> Bool,
        certified -> Nullable<Bool>,
        assignee_id -> Nullable<Int4>,
        repository_id -> Int4,
        issue_created_at -> Timestamptz,
        issue_closed_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        description -> Nullable<Text>,
        estimation -> Int4,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        slug -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    milestones (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        url -> Nullable<Text>,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Text,
        slug -> Text,
        types -> Nullable<Array<Nullable<Text>>>,
        purposes -> Nullable<Array<Nullable<Text>>>,
        stack_levels -> Nullable<Array<Nullable<Text>>>,
        technologies -> Nullable<Array<Nullable<Text>>>,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        rewards -> Bool,
    }
}

diesel::table! {
    repositories (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        url -> Text,
        language_slug -> Nullable<Text>,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        labels -> Nullable<Array<Nullable<Text>>>,
        url -> Nullable<Text>,
        milestone_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        avatar -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        github_id -> Nullable<Int8>,
    }
}

diesel::table! {
    users_milestones_role (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        project_id -> Int4,
    }
}

diesel::table! {
    users_tasks_role (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        milestone_id -> Int4,
    }
}

diesel::joinable!(issues -> repositories (repository_id));
diesel::joinable!(issues -> users (assignee_id));
diesel::joinable!(milestones -> projects (project_id));
diesel::joinable!(repositories -> projects (project_id));
diesel::joinable!(tasks -> milestones (milestone_id));
diesel::joinable!(users_milestones_role -> projects (project_id));
diesel::joinable!(users_milestones_role -> users (user_id));
diesel::joinable!(users_tasks_role -> milestones (milestone_id));
diesel::joinable!(users_tasks_role -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    issues,
    languages,
    milestones,
    projects,
    repositories,
    tasks,
    users,
    users_milestones_role,
    users_tasks_role,
);
