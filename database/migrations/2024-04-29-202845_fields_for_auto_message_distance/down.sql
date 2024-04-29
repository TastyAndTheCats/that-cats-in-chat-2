-- This file should undo anything in `up.sql`


ALTER TABLE twitch_user DROP messages_processed;
ALTER TABLE user_selected_responders DROP message_count_at_last_automatic;
