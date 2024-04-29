-- This file should undo anything in `up.sql`

DELETE FROM user_selected_responders WHERE responder_id = 113;
DELETE FROM user_selected_responders WHERE responder_id = 114;
DELETE FROM user_selected_responders WHERE responder_id = 115;
DELETE FROM user_selected_responders WHERE responder_id = 116;
DELETE FROM user_selected_responders WHERE responder_id = 117;
DELETE FROM user_selected_responders WHERE responder_id = 118;
DELETE FROM user_selected_responders WHERE responder_id = 119;
DELETE FROM twitch_bot_responders WHERE id = 119;
DELETE FROM twitch_bot_responders WHERE id = 118;
DELETE FROM twitch_bot_responders WHERE id = 113;
DELETE FROM twitch_bot_responders WHERE id = 114;
DELETE FROM twitch_bot_responders WHERE id = 115;
DELETE FROM twitch_bot_responders WHERE id = 116;
DELETE FROM twitch_bot_responders WHERE id = 117;