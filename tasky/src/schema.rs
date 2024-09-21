// @generated automatically by Diesel CLI.

diesel::table! {
    group_join_requests (id) {
        id -> Int4,
        requestor -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        title -> Varchar,
        members -> Array<Nullable<Int4>>,
        tutor -> Int4,
    }
}

diesel::joinable!(group_join_requests -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_join_requests,
    groups,
);
