use api_consumers::epic::free_games;
use chrono::{DateTime, NaiveDateTime, Utc};
use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;
use utils::serde_json::unwrap_reqwest;

pub async fn dispatch(
    responder: &TwitchResponder,
    _msg: &PrivmsgMessage,
    _command: &str,
) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("api::epic_store::free_games") {
        cmd_epic_store_free_games().await
    } else {
        String::from("Unknown Function (epic_store)")
    }
}

struct EpicFreeGameInfo {
    title: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    price: i32,
}

async fn cmd_epic_store_free_games() -> String {
    let games = get_games_list_from_egs_api().await;
    format!(
        "Currently available FOR FREE on the Epic Games Store: {}.
        Tasty collects these games, but he never plays them.",
        games.join(", ")
    )
}

async fn get_games_list_from_egs_api() -> Vec<String> {
    let games_list = get_data_from_api().await;
    let now = DateTime::from_timestamp(Utc::now().timestamp(), 0).unwrap();
    let game_list = build_dataset_from_api_data(games_list, now);
    let game_names_list = extract_useful_information(game_list, now);

    game_names_list
}

async fn get_data_from_api() -> Vec<serde_json::Value> {
    let raw_json = unwrap_reqwest(free_games().await).await;
    let fake_games_list: Vec<serde_json::Value> = Vec::new();
    let games_list: &Vec<serde_json::Value> = raw_json["data"]["Catalog"]["searchStore"]
        ["elements"]
        .as_array()
        .unwrap_or(&fake_games_list);
    games_list.to_vec()
}
fn build_dataset_from_api_data(
    games_list: Vec<serde_json::Value>,
    now: DateTime<Utc>,
) -> Vec<EpicFreeGameInfo> {
    let mut game_list: Vec<EpicFreeGameInfo> = Vec::new();
    for game in games_list {
        let promo = &game["promotions"]["promotionalOffers"][0]["promotionalOffers"][0];
        let price = &game["price"]["totalPrice"];
        match promo["startDate"].as_str() {
            Some(date) => {
                game_list.push(EpicFreeGameInfo {
                    title: game["title"].as_str().unwrap().to_string(),
                    start_date: DateTime::from_naive_utc_and_offset(
                        NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S.000Z").unwrap(),
                        now.offset().clone(),
                    ),
                    end_date: DateTime::from_naive_utc_and_offset(
                        NaiveDateTime::parse_from_str(
                            promo["endDate"].as_str().unwrap(),
                            "%Y-%m-%dT%H:%M:%S.000Z",
                        )
                        .unwrap(),
                        now.offset().clone(),
                    ),
                    price: price["discountPrice"].as_i64().unwrap().try_into().unwrap(),
                });
            }
            None => {}
        }
    }
    game_list
}
fn extract_useful_information(game_list: Vec<EpicFreeGameInfo>, now: DateTime<Utc>) -> Vec<String> {
    let mut game_names_list = Vec::new();
    for game in game_list {
        if game.price == 0 && game.start_date <= now && game.end_date >= now {
            game_names_list.push(game.title);
        } else {
            tracing::info!("This game isn't free at EGS right now: {}", game.title)
        }
    }
    game_names_list
}
