-- Your SQL goes here


ALTER TABLE twitch_user ADD messages_processed INTEGER NOT NULL default 0;
ALTER TABLE user_selected_responders ADD message_count_at_last_automatic INTEGER NOT NULL default 0;