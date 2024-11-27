use chrono::Utc;

use database::{
    models::responders::TwitchResponder,
    responder::{
        get_count, get_last_automatic_instance, update_count, update_last_automatic_instance,
    },
};
use twitch_irc::message::PrivmsgMessage;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, _command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("games::marbles::play") {
        cmd_play()
    } else if response_fn.starts_with("games::marbles::autoplay") {
        cmd_autoplay(responder, msg)
    } else {
        "Unknown Function (tribute)".to_owned()
    }
}

fn cmd_play() -> String {
    String::from("!play")
}

fn cmd_autoplay(responder: &TwitchResponder, msg: &PrivmsgMessage) -> String {
    let user_id = msg.sender.id.parse::<i32>().unwrap_or(0);
    let chatters_who_are_playing = get_count(user_id, responder.id).unwrap_or(0);
    let mut last_instance = get_last_automatic_instance(user_id, responder.id).unwrap_or(0);
    let two_minutes = 60 * 2;
    let one_hour = 60 * 60;
    let now = Utc::now().timestamp().try_into().unwrap_or(0);

    // If the last instance is more than an hour ago set it to now so it won't fire immediately
    if last_instance < now - one_hour {
        let _ = update_count(user_id, responder.id, 0);
        let _ = update_last_automatic_instance(user_id, responder.id);
        last_instance = now;
    }

    if now - two_minutes > last_instance {
        if chatters_who_are_playing > 5 {
            let _ = update_count(user_id, responder.id, 0);
            let _ = update_last_automatic_instance(user_id, responder.id);
            return cmd_play();
        } else {
            let _ = update_count(user_id, responder.id, chatters_who_are_playing + 1);
        }
    }
    String::new()
}
