use chrono::Utc;
use database::{models::responders::TwitchResponder, responder::get_last_instance};
use types::get::channel;

/// Checks if there has been an appropriate amount of time since the last message
pub fn check(responder: &TwitchResponder) -> bool {
    let cooldown: i32 = responder.cooldown;
    let user = channel(None, None);
    let last_instance = get_last_instance(user.id, responder.id).unwrap_or(0);
    let now = i32::try_from(Utc::now().timestamp()).expect("good until 2038");
    tracing::debug!("{} - {} >= {}", now, cooldown, last_instance);
    cooldown == 0 || last_instance == 0 || now - cooldown >= last_instance
}
