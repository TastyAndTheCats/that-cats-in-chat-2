use std::{env, time::Duration};

use chrono::Utc;
use database::{
    channel::get_messages_counted,
    models::responders::TwitchResponder,
    responder::{
        get_last_automatic_message_count, get_last_instance, update_last_automatic_instance,
        update_last_automatic_message_count, update_last_instance,
    },
};

use crate::{handler::privmsgs::send_response_or_run_response_fn, local_types::TwitchClient};

pub async fn scheduler(client: TwitchClient, responders: Vec<TwitchResponder>) {
    tracing::debug!("starting timed_message::scheduler");
    let mut loop_counter = 0;
    let user_id = env::var("TWITCH_CHANNEL_ID")
        .unwrap_or(String::from("0"))
        .parse::<i32>()
        .unwrap_or(0);
    let valid_responders = get_valid_responders(responders);
    init_responder_timestamps(&valid_responders, user_id);
    loop {
        loop_counter = loop_counter + 1;
        tracing::info!("starting timed_message::scheduler loop {}", loop_counter);
        run_scheduled_responses(&client, valid_responders.clone(), &user_id).await;
        tokio::time::sleep(Duration::new(5, 0)).await;
    }
}

fn init_responder_timestamps(responders: &Vec<TwitchResponder>, user_id: i32) {
    for r in responders {
        let _ = update_last_instance(user_id, r.id);
        let _ = update_last_automatic_instance(user_id, r.id);
        if let Ok(channel_message_count) = get_messages_counted(user_id) {
            let _ = update_last_automatic_message_count(user_id, r.id, channel_message_count);
        }
    }
}

async fn run_scheduled_responses(
    client: &TwitchClient,
    valid_responders: Vec<TwitchResponder>,
    user_id: &i32,
) {
    let loop_now: i32 = Utc::now().timestamp().try_into().unwrap_or(0); // current timestamp
    for responder in valid_responders.into_iter() {
        let channel_message_count = get_messages_counted(*user_id).unwrap_or(0);
        if check_responder_interval(user_id, &responder, loop_now)
            && check_responder_distance(user_id, &responder, channel_message_count)
        {
            // Set the necessary checks for next loop
            let _ = update_last_automatic_instance(*user_id, responder.id);
            let _ =
                update_last_automatic_message_count(*user_id, responder.id, channel_message_count);
            // Spawn a process to handle the responder's task
            let client = client.clone();
            tokio::spawn(async move {
                send_response_or_run_response_fn(client, responder, None, None).await;
            });
        }
    }
}

fn check_responder_interval(user_id: &i32, responder: &TwitchResponder, now: i32) -> bool {
    // If the cutoff timestamp is before now (i.e. bot is allowed to run command)
    let interval_compare = get_last_instance(*user_id, responder.id).unwrap_or(0)
        + responder.cooldown
        + responder.interval.unwrap(); // timestamp after which bot can re-run the responder.
    tracing::debug!(
        "{} interval: {} <= {} = {}",
        responder.title,
        interval_compare,
        now,
        interval_compare <= now
    );
    interval_compare <= now
}

fn check_responder_distance(
    user_id: &i32,
    responder: &TwitchResponder,
    current_height: i32,
) -> bool {
    // How many messages total should have been received since the last time this message was automatically sent
    let distance_compare = get_last_automatic_message_count(*user_id, responder.id).unwrap_or(0)
        + responder.distance.unwrap_or(0);

    tracing::debug!(
        "{} distance: {} <= {} = {}",
        responder.title,
        distance_compare,
        current_height,
        distance_compare <= current_height
    );
    distance_compare <= current_height
}

fn get_valid_responders(responders: Vec<TwitchResponder>) -> Vec<TwitchResponder> {
    let valid_responders: Vec<TwitchResponder> = responders
        .into_iter()
        .filter(|r| r.interval.unwrap_or(0) > 0 && r.distance.unwrap_or(0) > 0)
        .collect();

    tracing::debug!("Valid responders:");
    for r in valid_responders.clone().into_iter() {
        tracing::debug!("\tr: {}", r);
    }

    valid_responders
}
