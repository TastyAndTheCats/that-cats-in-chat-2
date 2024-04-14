//! Handles messages coming from the Twitch IRC stream
//! The `dispatch`` function should ideally be nothing but the match(es) required for managing the responses, db, or whatever is necessary.
//! I'm thinking that will be the only function or functions in here (e.g. dispatch_whispers, dispatch_privmsgs, etc.)
//! Everything else will either be a module or a responder or I guess just a pure db response for e.g watcher points

use tokio::{self, sync::mpsc::UnboundedReceiver};
use twitch_irc::message::ServerMessage;

/// Decides what sort of message is being received by the chatbot and what to do about it
pub async fn dispatch(mut incoming_messages: UnboundedReceiver<ServerMessage>) {
    while let Some(message) = incoming_messages.recv().await {
        match message {
            ServerMessage::Privmsg(msg) => {
                tracing::info!(
                    "#{} {}: {}",
                    msg.channel_login,
                    msg.sender.name,
                    msg.message_text
                );
            }

            ServerMessage::Whisper(msg) => {
                tracing::info!("Received whisper: {:?}", msg);
            }
            ServerMessage::Ping(msg) => {
                tracing::debug!("Pinged: {:?}", msg);
            }
            ServerMessage::Pong(msg) => {
                tracing::debug!("Ponged: {:?}", msg);
            }
            _ => {
                tracing::info!("Received something else: {:?}", message);
            }
        }
    }
}
