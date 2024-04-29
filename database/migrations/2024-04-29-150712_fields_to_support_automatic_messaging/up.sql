-- Your SQL goes here
ALTER TABLE user_selected_responders ADD last_automatic_instance INTEGER NOT NULL DEFAULT 0;

ALTER TABLE twitch_bot_responders ADD automatable BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE twitch_bot_responders ADD show_command_as VARCHAR(20);

INSERT INTO
    twitch_bot_auto_response_profiles (id, title, INTERVAL, distance)
VALUES
    (4, '1 min', 60, 50),
    (5, '2 min', 60 * 2, 50),
    (6, '3 min', 60 * 3, 50),
    (7, '5 min', 60 * 5, 50),
    (8, '10 min', 60 * 10, 50),
    (9, '15 min', 60 * 15, 50),
    (10, '20 min', 60 * 20, 50),
    (11, '25 min', 60 * 25, 50),
    (12, '30 min', 60 * 30, 50),
    (13, '45 min', 60 * 45, 50),
    (14, '75 min', 60 * 60 * 1.25, 50),
    (15, '90 min', 60 * 60 * 1.5, 50),
    (16, '2 hours', 60 * 60 * 2, 50),
    (17, '3 hours', 60 * 60 * 3, 50),
    (18, '4 hours', 60 * 60 * 4, 50),
    (19, '5 hours', 60 * 60 * 5, 50),
    (20, '6 hours', 60 * 60 * 6, 50),
    (21, '8 hours', 60 * 60 * 8, 50),
    (22, '12 hours', 60 * 60 * 12, 50);