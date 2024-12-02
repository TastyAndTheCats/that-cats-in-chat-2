-- This file should undo anything in `up.sql`

ALTER TABLE twitch_user DROP COLUMN messages_processed;
ALTER TABLE user_selected_responders DROP COLUMN message_count_at_last_automatic;
