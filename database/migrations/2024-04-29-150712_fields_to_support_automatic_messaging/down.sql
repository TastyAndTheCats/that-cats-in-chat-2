-- This file should undo anything in `up.sql`

ALTER TABLE user_selected_responders DROP 
    last_automatic_instance
;

ALTER TABLE twitch_bot_responders DROP automatable;
ALTER TABLE twitch_bot_responders DROP show_command_as;