//! Handles responses to normal chat messages
use std::env;

use crate::local_types::TwitchClient;
use crate::responder;
use api_consumers::twitch::users::lookup_user_from_login;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::message::rest_of_chat_message;
use utils::serde_json::unwrap_reqwest;

mod automoderator;

/// Dispatches all of the chatbot responses. This is the main brain of the chatbot's response engine.
pub async fn dispatch(
    client: &TwitchClient,
    msg: PrivmsgMessage,
    responders: &Vec<TwitchResponder>,
) {
    if automoderator::check(&msg.message_text) {
        let m = msg.message_text.to_lowercase();
        for r in responders {
            if r.starts_with.is_some() {
                for opt in r.starts_with.as_ref().unwrap().split("|") {
                    if m.starts_with(opt) {
                        send_response_or_run_response_fn(client, &r, &msg, opt).await;
                    }
                }
            }

            if r.contains.is_some() {
                for opt in r.contains.as_ref().unwrap().split("|") {
                    if m.contains(opt) {
                        send_response_or_run_response_fn(client, &r, &msg, opt).await;
                    }
                }
            }

            if r.ends_with.is_some() {
                for opt in r.ends_with.as_ref().unwrap().split("|") {
                    if m.ends_with(opt) {
                        send_response_or_run_response_fn(client, &r, &msg, opt).await;
                    }
                }
            }
        }
    }
}

async fn send_response_or_run_response_fn(
    client: &TwitchClient,
    r: &TwitchResponder,
    msg: &PrivmsgMessage,
    command: &str,
) {
    responder::send(
        client,
        Some(msg),
        match r.response_fn.is_some() {
            true => {
                insert_data_in_response(
                    responder::function_message(r, msg, command).await,
                    msg,
                    command,
                )
                .await
            }
            false => format_response(&r.response.as_ref().unwrap().to_owned(), msg, command).await,
        },
        Some(r),
    )
    .await;
}

/// Formats the response message for Twitch chat
async fn format_response(r: &String, msg: &PrivmsgMessage, command: &str) -> String {
    insert_data_in_response(r.to_owned(), msg, command).await
}

/// Rwplaces text variables (no format yet) with the real data where possible
async fn insert_data_in_response(response: String, msg: &PrivmsgMessage, command: &str) -> String {
    let mut response = replace_sender_name(response, msg);
    response = replace_channel_name(response, msg).await;
    replace_1_(response, msg, command)
}

/// Replace {sender} with message sender display name
fn replace_sender_name(response: String, msg: &PrivmsgMessage) -> String {
    response.replace("{sender}", &msg.sender.name)
}

/// Replace {channel_name} with channel display name
async fn replace_channel_name(response: String, msg: &PrivmsgMessage) -> String {
    let user = unwrap_reqwest(lookup_user_from_login(&msg.channel_login).await).await;
    println!("{}", user);
    response.replace(
        "{channel_name}",
        user["data"][0]["display_name"]
            .as_str()
            .unwrap_or(&env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL is missing")),
    )
}

/// replace {_1}
/// NOTE: probably don't do this? it's for old commands that I don't want to update for whatever reason
fn replace_1_(response: String, msg: &PrivmsgMessage, command: &str) -> String {
    let rest_of_message = rest_of_chat_message(msg, command);
    response.replace("{_1}", &rest_of_message)
}

// PrivmsgMessage {
//     channel_login: "tastyandthecats".to_owned(),
//     channel_id: "167591621".to_owned(),
//     message_text: "!dwarfme".to_owned(),
//     is_action: false,
//     sender: TwitchUserBasics { id: "167591621", login: "tastyandthecats", name: "TastyAndTheCats" },
//     badge_info: [Badge { name: "subscriber", version: "44" }],
//     badges: [Badge { name: "broadcaster", version: "1" }, Badge { name: "subscriber", version: "3030" }, Badge { name: "rplace-2023", version: "1" }],
//     bits: None,
//     name_color: Some(RGBColor { r: 255, g: 164, b: 0 }),
//     emotes: [].to_vec(),
//     message_id: "9937b03d-a1dc-4b70-a2cc-09611487acf8".to_owned(),
//     server_timestamp: "2024-04-20T19:06:09.152Z".to_owned(),
//     source: IRCMessage {
// //         tags: IRCTags({
//                 "color": Some("#FFA400"),
//                 "user-type": Some(""),
//                 "id": Some("9937b03d-a1dc-4b70-a2cc-09611487acf8"),
//                 "client-nonce": Some("1a443422ce2c4b74cbaf0653272fb962"),
//                 "returning-chatter": Some("0"),
//                 "badge-info": Some("subscriber/44"),
//                 "emotes": Some(""),
//                 "tmi-sent-ts": Some("1713639969152"),
//                 "badges": Some("broadcaster/1,subscriber/3030,rplace-2023/1"),
//                 "subscriber": Some("1"),
//                 "flags": Some(""),
//                 "display-name": Some("TastyAndTheCats"),
//                 "turbo": Some("0"),
//                 "first-msg": Some("0"),
//                 "user-id": Some("167591621"),
//                 "room-id": Some("167591621"),
//                 "mod": Some("0")
//             }),
//         prefix: Some(Full { nick: "tastyandthecats", user: Some("tastyandthecats"), host: Some("tastyandthecats.tmi.twitch.tv") }),
//         command: "PRIVMSG",
//         params: ["#tastyandthecats", "!dwarfme"]
//     }
// }
