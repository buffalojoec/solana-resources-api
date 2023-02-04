// @generated automatically by Diesel CLI.

diesel::table! {
    resources (id) {
        id -> Uuid,
        resource_scope -> Text,
        resource_topic -> Text,
        resource_title -> Text,
        resource_type -> Text,
        resource_url -> Text,
    }
}
