// @generated automatically by Diesel CLI.

diesel::table! {
    microphones (id) {
        id -> Integer,
        model -> Text,
        manufacturer -> Text,
        description -> Text,
        main_image -> Nullable<Text>,
    }
}
