//! Handles responses to normal chat messages

use crate::responder;
use crate::types::TwitchClientType;
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;

pub async fn dispatch(
    client: &TwitchClientType,
    msg: PrivmsgMessage,
    responders: &Vec<TwitchResponder>,
) {
    for r in responders {
        if r.starts_with.is_some()
            && msg
                .message_text
                .starts_with(&*r.starts_with.as_ref().unwrap())
        {
            responder::send(client, &r.response).await;
        } else if r.ends_with.is_some()
            && msg.message_text.ends_with(&*r.ends_with.as_ref().unwrap())
        {
            responder::send(client, &r.response).await;
        } else if r.contains.is_some() && msg.message_text.contains(&*r.contains.as_ref().unwrap())
        {
            responder::send(client, &r.response).await;
        }
    }
}
