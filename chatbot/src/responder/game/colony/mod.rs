use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;

use crate::types::TwitchClientType;

pub async fn dispatch(
    client: &TwitchClientType,
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

// async fn cmd_shoutout(client: &TwitchClientType, msg: &PrivmsgMessage, command: &str) -> String {
//     let user_json: serde_json::Value = serde_json::from_str(
//         &lookup_user_from_login(&single_word_after_command(msg, command))
//             .await
//             .text()
//             .await
//             .unwrap(),
//     )
//     .unwrap();

//     let user = &user_json["data"][0];
//     let so_id = user["id"].as_str().unwrap();

//     let channel_json: serde_json::Value =
//         serde_json::from_str(&lookup_channel_from_id(so_id).await.text().await.unwrap()).unwrap();
//     let channel = &channel_json["data"][0];

//     let game_name = channel["game_name"].as_str().unwrap();
//     let so_name = user["display_name"].as_str().unwrap();
//     let so_description = user["description"].as_str().unwrap();

//     let mut game_message = format!("{} hasn't streamed (yet), but they might!", so_name);
//     if game_name != "" {
//         game_message = format!("{} was last seen streaming {}!", so_name, game_name);
//     }

//     do_builtin_twitch_shoutout(client, msg, so_name).await;
//     format!(
//         "Go check out {} at twitch.tv/{} and make a new friend?! {} ⟹⟹⟹ {}",
//         so_name, so_name, game_message, so_description,
//     )
// }

// async fn do_builtin_twitch_shoutout(
//     client: &TwitchClientType,
//     msg: &PrivmsgMessage,
//     command: &str,
// ) {
//     // NOTE: Doesn't seem to work
//     client
//         .privmsg(
//             msg.channel_login.to_owned(),
//             format!("/shoutout {}", command),
//         )
//         .await
//         .unwrap();
// }
