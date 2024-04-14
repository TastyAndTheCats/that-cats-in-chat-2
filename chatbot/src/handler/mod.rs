use tokio::{self, sync::mpsc::UnboundedReceiver};
use twitch_irc::message::ServerMessage;

pub async fn handle_twitch_message(mut incoming_messages: UnboundedReceiver<ServerMessage>) {
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
