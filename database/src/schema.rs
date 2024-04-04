diesel::table! {
    twitch_channel (id) {
        id -> Text,
        login -> Text,
        name -> Text,
        language -> Text,
        game_id -> Text,
        title -> Text, // Last Stream
        delay -> Int,
        content_classification -> Text,
        is_branded_content -> Boolean,
    }
}

diesel::table! {
    twitch_games (id) {
        id -> Text,
        name -> Text,
    }
}