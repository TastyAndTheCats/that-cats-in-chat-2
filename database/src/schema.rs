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
    twitch_bot_auto_response_profiles (id) {
        id -> Int4,
        title -> Text,
        interval -> Nullable<Int4>,
        distance -> Nullable<Int4>,
    }
}

diesel::table! {
    twitch_bot_responder_groups (id) {
        id -> Int4,
        #[max_length = 1000]
        title -> Varchar,
        active -> Bool,
        parent -> Nullable<Int4>,
    }
}

diesel::table! {
    twitch_bot_responders (id) {
        id -> Int4,
        responder_group_id -> Nullable<Int4>,
        #[max_length = 1000]
        title -> Varchar,
        active -> Bool,
        #[max_length = 500]
        starts_with -> Nullable<Varchar>,
        #[max_length = 500]
        ends_with -> Nullable<Varchar>,
        #[max_length = 500]
        contains -> Nullable<Varchar>,
        #[max_length = 500]
        response -> Nullable<Varchar>,
        #[max_length = 500]
        response_fn -> Nullable<Varchar>,
        requires_broadcaster -> Bool,
        requires_moderator -> Bool,
        requires_vip -> Bool,
        requires_subscriber -> Bool,
        requires_follower -> Bool,
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
    user_selected_modules (user_id, responder_group_id) {
        user_id -> Int4,
        responder_group_id -> Int4,
        active -> Bool,
    }
}

diesel::table! {
    user_selected_responders (user_id, responder_id) {
        user_id -> Int4,
        responder_id -> Int4,
        responder_profile -> Int4,
        active -> Bool,
        last_instance -> Int4,
        cooldown -> Int4,
        per_user_cooldown -> Int4,
    }
}

diesel::joinable!(twitch_bot -> twitch_user (channel_id));
diesel::joinable!(twitch_bot_responders -> twitch_bot_responder_groups (responder_group_id));
diesel::joinable!(twitch_user -> twitch_login_process (login_state));
diesel::joinable!(user_selected_modules -> twitch_bot_responder_groups (responder_group_id));
diesel::joinable!(user_selected_modules -> twitch_user (user_id));
diesel::joinable!(user_selected_responders -> twitch_bot_auto_response_profiles (responder_profile));
diesel::joinable!(user_selected_responders -> twitch_bot_responders (responder_id));
diesel::joinable!(user_selected_responders -> twitch_user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    twitch_bot,
    twitch_bot_auto_response_profiles,
    twitch_bot_responder_groups,
    twitch_bot_responders,
    twitch_login_process,
    twitch_user,
    user_selected_modules,
    user_selected_responders,
);
