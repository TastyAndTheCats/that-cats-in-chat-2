mod epic_store;
mod oeis;
mod ollama;
mod openweathermap;
mod twitch;
mod wikipedia;
mod words;
mod youtube;

use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    tracing::info!("{} '{}'", response_fn, command);
    if response_fn.starts_with("api::epic_store") {
        return epic_store::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::oeis") {
        return oeis::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::openweathermap") {
        return openweathermap::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::ollama") {
        return ollama::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::twitch") {
        return twitch::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::wikipedia") {
        return wikipedia::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::words") {
        return words::dispatch(responder, msg, command).await;
    } else if response_fn.starts_with("api::youtube") {
        return youtube::dispatch(responder, msg, command).await;
    } else {
        return "Unknown Function (api)".to_owned();
    }
}
