
-- INSERT INTO
--     twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, contains, ends_with, response_fn)
-- VALUES
--     (113, 6, 'Epic Store Free Games', '!epic|!epicfree', 'epic games store', 'epic?', 'api::epic_store::free_games');

-- INSERT INTO user_selected_responders (user_id, responder_id, permissions) VALUES (167591621, 113, 4);

-- NOTES:
-- PERMISSIONS
-- 1 = Any
-- 2 = Broadcaster
-- 3 = Broadcaster, Mod
-- 4 = Broadcaster, Mods, VIP*
-- 5 = Sub+
-- 6 = Follower+*
-- 7 = Mod-Only
-- 8 = VIP-Only*
-- 9 = Sub-only
-- 10 = Follower-only*
-- there's no code for VIP or followers yet so that's an option but not a reality