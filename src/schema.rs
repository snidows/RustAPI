// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        city -> Varchar,
        cep -> Varchar,
        tel -> Varchar,
    }
}
