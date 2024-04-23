// @generated automatically by Diesel CLI.

diesel::table! {
    student (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        subjects -> Nullable<Array<Nullable<Text>>>,
    }
}
