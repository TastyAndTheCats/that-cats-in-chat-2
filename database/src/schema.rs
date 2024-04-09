// @generated automatically by Diesel CLI.

diesel::table! {
    twitch_channel (id) {
        id -> Text,
        login -> Text,
        name -> Text,
        language -> Text,
        delay -> Int4,
    }
}

diesel::table! {
    twitch_login_process (state) {
        state -> Text,
        scope -> Text,
        code -> Nullable<Text>,
        is_bot -> Bool,
        is_broadcaster -> Bool,
        initiated_at -> Timestamp,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        token_type -> Nullable<Text>,
        token_expiry -> Nullable<Int8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    twitch_channel,
    twitch_login_process,
);
