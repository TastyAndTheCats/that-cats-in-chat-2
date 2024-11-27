-- Your SQL goes here
ALTER TABLE user_selected_responders ADD last_automatic_instance INTEGER NOT NULL DEFAULT 0;

ALTER TABLE twitch_bot_responders ADD automatable BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE twitch_bot_responders ADD show_command_as VARCHAR(20);