use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;

use crate::local_types::TwitchClient;

pub async fn dispatch(
    client: &TwitchClient,
    responder: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("games::colony::tribute") {
        return cmd_tribute(msg, command).await;
    } else {
        return "Unknown Function (tribute)".to_owned();
    }
}

async fn cmd_tribute(msg: &PrivmsgMessage, command: &str) -> String {
    match command {
        "!tribute" => {
            return format!(
                "\"I volunteer as tribute!\" screams a wild-eyed {}",
                msg.sender.name
            );
        }
        "!dwarfme" => {
            return format!(
                "{} wants to be a short, sturdy creature, fond of drink and industry.",
                msg.sender.name
            );
        }
        "!volunteer" | _ => {
            return format!("{} says \"Put me in, coach!\"", msg.sender.name);
        }
    }
}
