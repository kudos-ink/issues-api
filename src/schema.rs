// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tip_status"))]
    pub struct TipStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tip_type"))]
    pub struct TipType;
}

diesel::table! {
    blockchains (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        wish_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        comment -> Nullable<Varchar>,
        positive_votes -> Nullable<Int4>,
        negative_votes -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    filter_values (id) {
        id -> Int4,
        filters_id -> Int4,
        emoji -> Text,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    filters (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        emoji -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    issues (id) {
        id -> Int4,
        issue_number -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamp>,
        repository_id -> Nullable<Int4>,
    }
}

diesel::table! {
    issues_labels (id) {
        id -> Int4,
        issue_id -> Nullable<Int4>,
        labels_id -> Nullable<Int4>,
    }
}

diesel::table! {
    labels (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

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
    maintainers (id) {
        id -> Int4,
        repository_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    repositories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        organization_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    repositories_filters (id) {
        id -> Int4,
        repositories_id -> Nullable<Int4>,
        filters_id -> Nullable<Int4>,
        filter_values_id -> Nullable<Int4>,
    }
}

diesel::table! {
    repositories_languages (id) {
        id -> Int4,
        repositories_id -> Nullable<Int4>,
        languages_id -> Nullable<Int4>,
    }
}

diesel::table! {
    repositories_topics (id) {
        id -> Int4,
        repositories_id -> Nullable<Int4>,
        topics_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TipStatus;
    use super::sql_types::TipType;

    tips (id) {
        id -> Int4,
        status -> TipStatus,
        #[sql_name = "type"]
        type_ -> TipType,
        amount -> Int8,
        #[max_length = 48]
        to -> Varchar,
        #[max_length = 48]
        from -> Varchar,
        #[max_length = 255]
        transaction -> Nullable<Varchar>,
        blockchain_id -> Nullable<Int4>,
        #[max_length = 255]
        url -> Nullable<Varchar>,
        contributor_id -> Int4,
        curator_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    topics (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    wishes (id) {
        id -> Int4,
        issues_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(comments -> users (user_id));
diesel::joinable!(comments -> wishes (wish_id));
diesel::joinable!(filter_values -> filters (filters_id));
diesel::joinable!(issues -> repositories (repository_id));
diesel::joinable!(issues_labels -> issues (issue_id));
diesel::joinable!(issues_labels -> labels (labels_id));
diesel::joinable!(maintainers -> repositories (repository_id));
diesel::joinable!(maintainers -> users (user_id));
diesel::joinable!(repositories -> organizations (organization_id));
diesel::joinable!(repositories_filters -> filter_values (filter_values_id));
diesel::joinable!(repositories_filters -> filters (filters_id));
diesel::joinable!(repositories_filters -> repositories (repositories_id));
diesel::joinable!(repositories_languages -> languages (languages_id));
diesel::joinable!(repositories_languages -> repositories (repositories_id));
diesel::joinable!(repositories_topics -> repositories (repositories_id));
diesel::joinable!(repositories_topics -> topics (topics_id));
diesel::joinable!(tips -> blockchains (blockchain_id));
diesel::joinable!(wishes -> issues (issues_id));

diesel::allow_tables_to_appear_in_same_query!(
    blockchains,
    comments,
    filter_values,
    filters,
    issues,
    issues_labels,
    labels,
    languages,
    maintainers,
    organizations,
    repositories,
    repositories_filters,
    repositories_languages,
    repositories_topics,
    tips,
    topics,
    users,
    wishes,
);
