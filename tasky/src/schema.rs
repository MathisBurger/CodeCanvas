// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        title -> Varchar,
        members -> Array<Nullable<Int4>>,
        tutor -> Int4,
    }
}
