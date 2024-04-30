//! Handles messages coming from the Twitch IRC stream
//! The `dispatch`` function should ideally be nothing but the match(es) required for managing the responses, db, or whatever is necessary.
//! I'm thinking that will be the only function or functions in here
//! Everything else will either be a module or a responder or I guess just a pure db response for e.g watcher points

use database::models::responders::TwitchResponder;
use tokio::{self, sync::mpsc::UnboundedReceiver};
use twitch_irc::message::{FollowersOnlyMode, ServerMessage};

pub mod privmsgs;
use crate::local_types::TwitchClient;

/// Decides what sort of message is being received by the chatbot and what to do about it
pub async fn dispatch(
    client: TwitchClient,
    mut incoming_messages: UnboundedReceiver<ServerMessage>,
    responders: Vec<TwitchResponder>,
     bot_id: i32,
) {
    while let Some(message) = incoming_messages.recv().await {
        match message {
            ServerMessage::Generic(msg) => {
                tracing::debug!("{:?}", msg)
            }
            ServerMessage::GlobalUserState(msg) => {
                tracing::info!("{} is here", msg.user_name);
            }
            ServerMessage::Join(msg) => {
                tracing::info!("{} joined {}", msg.user_login, msg.channel_login);
            }
            ServerMessage::Ping(_msg) => {
                tracing::debug!("|.     ");
            }
            ServerMessage::Pong(_msg) => {
                tracing::debug!("     .|");
            }
            ServerMessage::Privmsg(msg) => {
                tracing::info!(
                    "#{} {}: {}",
                    &msg.channel_login,
                    &msg.sender.name,
                    &msg.message_text
                );
                if msg.sender.id != format!("{}", bot_id) {
                    privmsgs::dispatch(client.clone(), msg, responders.clone()).await;
                }
            }
            ServerMessage::RoomState(msg) => {
                tracing::debug!(
                    "{}:#{} {} {}",
                    msg.channel_id,
                    msg.channel_login,
                    if msg.subscribers_only.unwrap_or(false) {
                        "subscriber-only chat"
                    } else if msg.follwers_only.unwrap_or(FollowersOnlyMode::Disabled)
                        != FollowersOnlyMode::Disabled
                    {
                        "follower-only chat"
                    } else {
                        "any-message"
                    },
                    if msg.emote_only.unwrap_or(false) {
                        "emote-only"
                    } else {
                        "any-text"
                    }
                );
            }
            ServerMessage::UserState(msg) => {
                tracing::info!("{} is here", msg.user_name);
            }
            ServerMessage::Whisper(msg) => {
                tracing::info!("Received whisper: {:?}", msg);
            }
            _ => {
                tracing::info!("Received something else: {:?}", message);
            }
        }
    }
}
