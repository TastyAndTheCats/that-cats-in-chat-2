// @generated automatically by Diesel CLI.

diesel::table! {
    twitch_bot (state) {
        state -> Text,
        id -> Nullable<Int4>,
        login -> Nullable<Text>,
        channel_id -> Int4,
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

diesel::table! {
    twitch_user (id) {
        id -> Int4,
        login -> Text,
        login_state -> Nullable<Text>,
    }
}

diesel::joinable!(twitch_bot -> twitch_user (channel_id));
diesel::joinable!(twitch_user -> twitch_login_process (login_state));

diesel::allow_tables_to_appear_in_same_query!(twitch_bot, twitch_login_process, twitch_user,);
