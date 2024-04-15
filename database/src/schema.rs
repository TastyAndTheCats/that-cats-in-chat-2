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
    twitch_bot_responder_groups (id) {
        id -> Int4,
        #[max_length = 1000]
        title -> Varchar,
        active -> Nullable<Bool>,
        parent -> Nullable<Int4>,
    }
}

diesel::table! {
    twitch_bot_responders (id) {
        id -> Int4,
        responder_group_id -> Nullable<Int4>,
        #[max_length = 1000]
        title -> Varchar,
        active -> Nullable<Bool>,
        #[max_length = 500]
        starts_with -> Nullable<Varchar>,
        #[max_length = 500]
        ends_with -> Nullable<Varchar>,
        #[max_length = 500]
        contains -> Nullable<Varchar>,
        #[max_length = 500]
        response -> Varchar,
        requires_broadcaster -> Nullable<Bool>,
        requires_moderator -> Nullable<Bool>,
        requires_vip -> Nullable<Bool>,
        requires_subscriber -> Nullable<Bool>,
        requires_follower -> Nullable<Bool>,
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

diesel::table! {
    user_selected_modules (user_id) {
        user_id -> Int8,
        responder_group_id -> Nullable<Int4>,
        active -> Nullable<Bool>,
    }
}

diesel::table! {
    user_selected_responders (user_id) {
        user_id -> Int8,
        responder_id -> Nullable<Int4>,
        active -> Nullable<Bool>,
    }
}

diesel::joinable!(twitch_bot -> twitch_user (channel_id));
diesel::joinable!(twitch_bot_responders -> twitch_bot_responder_groups (responder_group_id));
diesel::joinable!(twitch_user -> twitch_login_process (login_state));
diesel::joinable!(user_selected_modules -> twitch_bot_responder_groups (responder_group_id));
diesel::joinable!(user_selected_modules -> twitch_user (user_id));
diesel::joinable!(user_selected_responders -> twitch_bot_responders (responder_id));
diesel::joinable!(user_selected_responders -> twitch_user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    twitch_bot,
    twitch_bot_responder_groups,
    twitch_bot_responders,
    twitch_login_process,
    twitch_user,
    user_selected_modules,
    user_selected_responders,
);
