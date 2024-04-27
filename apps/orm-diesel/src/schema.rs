// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    t_blueprint (blueprint_id) {
        blueprint_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 64]
        user_id -> Varchar,
        config -> Text,
        state -> Bool,
        create_time -> Nullable<Timestamptz>,
        update_time -> Nullable<Timestamptz>,
        is_deleted -> Bool,
        delete_time -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    t_blueprint,
);
