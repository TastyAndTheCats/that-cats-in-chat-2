INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, contains, ends_with, response_fn)
VALUES
    (113, 6, 'Full Weather', '!maxweather', NULL, NULL, 'api::openweathermap::weather_full'),
    (114, 4, 'Chat with Friday', '!askf', NULL, NULL, 'api::ollama::friday'),
    (115, 4, 'Chat with Zoey', '!askz', NULL, NULL, 'api::ollama::zoey'),
    (116, 4, 'Chat with A Random Cat', '!catchat', NULL, NULL, 'api::ollama::chat'),
    (117, 6, 'Start a Fight', NULL, '!fight', NULL, 'core::facts::fight'),
    (118, 6, 'Roll a Die (1d20 default)', '!roll', NULL, NULL, 'core::maths::roll_die'),
    (119, 6, 'Oranges', '!orange|!oranges', NULL, NULL, 'core::emoji::fruit::oranges');

INSERT INTO
    user_selected_responders (user_id, responder_id, permissions)
VALUES
    (167591621, 113, 1),
    (167591621, 114, 1),
    (167591621, 115, 1),
    (167591621, 116, 1),
    (167591621, 117, 1),
    (167591621, 118, 1),
    (167591621, 119, 1);

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