-- This file should undo anything in `up.sql`

DELETE FROM user_selected_responders WHERE responder_id = 113;
DELETE FROM user_selected_responders WHERE responder_id = 114;
DELETE FROM user_selected_responders WHERE responder_id = 115;
DELETE FROM twitch_bot_responders WHERE id = 113;
DELETE FROM twitch_bot_responders WHERE id = 114;
DELETE FROM twitch_bot_responders WHERE id = 115;