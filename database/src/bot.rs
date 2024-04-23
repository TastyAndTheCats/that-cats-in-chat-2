use diesel::{prelude::*, result};
use types::get::channel;

use crate::{
    establish_connection,
    models::{responders::TwitchResponder, LoginProcess},
    schema::{
        twitch_bot_auto_response_profiles, twitch_bot_responder_permissions, twitch_bot_responders,
        twitch_login_process, twitch_user, user_selected_responders,
    },
};

// pub async fn get_responders_for_user(user_id: i32) -> Result<Vec<TwitchResponder>, result::Error> {
//     get_combined_responders_for_user(user_id).unwrap();
//     twitch_bot_responders::table
//         .inner_join(user_selected_responders::table)
//         .filter(user_selected_responders::user_id.eq(user_id))
//         .select(TwitchResponder::as_select())
//         .load(&mut establish_connection())
// }

pub fn get_access_token(id: Option<i32>) -> Result<LoginProcess, result::Error> {
    twitch_user::table
        .inner_join(twitch_login_process::table)
        .filter(twitch_user::id.eq(id.unwrap_or(channel(None, None).id)))
        .select(LoginProcess::as_select())
        .get_result(&mut establish_connection())
}

pub fn get_combined_responders_for_user(
    user_id: i32,
) -> Result<Vec<TwitchResponder>, result::Error> {
    user_selected_responders::table
        .inner_join(twitch_bot_responders::table)
        .inner_join(twitch_bot_responder_permissions::table)
        .inner_join(twitch_bot_auto_response_profiles::table)
        .filter(user_selected_responders::user_id.eq(user_id))
        .filter(user_selected_responders::active.eq(true))
        .filter(twitch_bot_responders::active.eq(true))
        .select((
            user_selected_responders::responder_id,
            user_selected_responders::responder_profile,
            user_selected_responders::last_instance,
            user_selected_responders::permissions,
            user_selected_responders::cooldown,
            // user_selected_responders::per_user_cooldown, // TODO: I need to keep track of users for this and I don't yet
            user_selected_responders::include_specific_users,
            user_selected_responders::exclude_specific_users,
            twitch_bot_auto_response_profiles::interval,
            twitch_bot_auto_response_profiles::distance,
            twitch_bot_responder_permissions::requires_broadcaster,
            twitch_bot_responder_permissions::requires_moderator,
            twitch_bot_responder_permissions::requires_vip,
            twitch_bot_responder_permissions::requires_subscriber,
            // // twitch_bot_responder_permissions::requires_follower, // TODO: this also requires I keep track of users (following is part of an api call not the msg) and I don't yet
            twitch_bot_responders::title,
            twitch_bot_responders::starts_with,
            twitch_bot_responders::contains,
            twitch_bot_responders::ends_with,
            twitch_bot_responders::response,
            twitch_bot_responders::response_fn,
        ))
        .get_results(&mut establish_connection())
}
