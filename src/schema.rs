// @generated automatically by Diesel CLI.

diesel::table! {
    menu (id) {
        id -> Int4,
        date -> Text,
        meal -> Text,
        students -> Array<Nullable<Text>>,
        price -> Int4,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    menu,
    students,
);
