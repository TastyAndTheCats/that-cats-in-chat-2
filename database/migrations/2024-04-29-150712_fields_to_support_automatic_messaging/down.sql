-- This file should undo anything in `up.sql`

ALTER TABLE user_selected_responders DROP 
    last_automatic_instance
;

ALTER TABLE twitch_bot_responders DROP automatable;
ALTER TABLE twitch_bot_responders DROP show_command_as;

UPDATE user_selected_responders SET responder_profile = 3 WHERE responder_profile > 3;

DELETE FROM twitch_bot_auto_response_profiles WHERE id = 4;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 5;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 6;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 7;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 8;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 9;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 10;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 11;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 12;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 13;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 14;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 15;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 16;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 17;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 18;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 19;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 20;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 21;
DELETE FROM twitch_bot_auto_response_profiles WHERE id = 22;