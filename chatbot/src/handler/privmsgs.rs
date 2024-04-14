//! Handles responses to normal chat messages

use crate::definitions::types::TwitchClientType;
use crate::responder::test_command;
use twitch_irc::message::PrivmsgMessage;

pub async fn dispatch(client: &TwitchClientType, msg: PrivmsgMessage) {
    if msg.message_text.contains("!test") {
        test_command(client).await;
    }
}
