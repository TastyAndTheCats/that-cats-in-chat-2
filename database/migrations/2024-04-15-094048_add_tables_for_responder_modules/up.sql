-- Definitions of modules
CREATE TABLE
    twitch_bot_responder_groups (
        id SERIAL PRIMARY KEY,
        title VARCHAR(1000) NOT NULL,
        active BOOLEAN NOT NULL DEFAULT TRUE,
        parent INTEGER,
        FOREIGN KEY (parent) REFERENCES twitch_bot_responder_groups (id),
        UNIQUE (title, parent)
    );

INSERT INTO
    twitch_bot_responder_groups (id, title)
VALUES
    (1, 'Core Functions'),
    (2, 'Game-Related'),
    (3, 'Third-Party');

INSERT INTO
    twitch_bot_responder_groups (id, title, parent)
VALUES
    (4, 'API Consumers', 3),
    (5, 'User-Defined', 3),
    (6, 'Epic Games Store', 4);

-- Definitions of responders
CREATE TABLE
    twitch_bot_responders (
        id SERIAL PRIMARY KEY,
        responder_group_id INTEGER REFERENCES twitch_bot_responder_groups (id),
        title VARCHAR(1000) NOT NULL,
        active BOOLEAN NOT NULL DEFAULT TRUE,
        STARTS_WITH VARCHAR(500),
        ends_with VARCHAR(500),
        contains VARCHAR(500),
        CONSTRAINT must_have_trigger CHECK (
            STARTS_WITH IS NOT NULL OR
            ends_with IS NOT NULL OR
            contains IS NOT NULL
        ),
        response VARCHAR(500),
        response_fn VARCHAR(500),
        CONSTRAINT must_have_response CHECK (
            response IS NOT NULL OR
            response_fn IS NOT NULL
        ),
        requires_broadcaster BOOLEAN NOT NULL DEFAULT FALSE,
        requires_moderator BOOLEAN NOT NULL DEFAULT FALSE,
        requires_vip BOOLEAN NOT NULL DEFAULT FALSE,
        requires_subscriber BOOLEAN NOT NULL DEFAULT FALSE,
        requires_follower BOOLEAN NOT NULL DEFAULT FALSE,
        UNIQUE (responder_group_id, title)
    );

-- just response
INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, response, requires_broadcaster)
VALUES
    (1, 1, 'Say Hello', 'hello!', 'HeyGuys', TRUE),
    (2, 1, '!test Command', '!test', 'TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE', TRUE);

-- just response_fn starts_with
INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, contains, ends_with, response_fn, requires_broadcaster)
VALUES
    (3, 1, '!fntest Command', NULL, '!fntest', NULL, 'unpack_the_galaxy', TRUE),
    (5, 6, 'Epic Store Free Games', '!epic|!epicfree', 'epic games store', 'epic?|epic', 'api::epic_store::free_games', TRUE);

-- response and response_fn
INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, response, response_fn, requires_broadcaster)
VALUES
    (4, 1, '!bothtest Command', '!bothtest', 'This is a test of using a response and a response function', 'default', TRUE);

-- Allow users to turn on and off whole modules
CREATE TABLE
    user_selected_modules (
        user_id INTEGER REFERENCES twitch_user (id),
        responder_group_id INTEGER REFERENCES twitch_bot_responder_groups (id) NOT NULL,
        active BOOLEAN NOT NULL DEFAULT TRUE,
        PRIMARY KEY (user_id, responder_group_id)
    );

INSERT INTO
    user_selected_modules (user_id, responder_group_id)
VALUES
    (167591621, 1);

-- Different auto-response profiles
CREATE TABLE
    twitch_bot_auto_response_profiles (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        INTERVAL INTEGER DEFAULT 3600, -- minimum seconds before this auto-response fires again
        distance INTEGER DEFAULT 50 -- minimum non-bot messages between repeat instances of this message
    );

INSERT INTO
    twitch_bot_auto_response_profiles (id, title, INTERVAL, distance)
VALUES
    (1, 'Never', NULL, NULL),
    (2, 'Hourly', 3600, 50),
    (3, 'Daily', 86400, 50);

-- Allow users to turn on and off specific responders in a module
CREATE TABLE
    user_selected_responders (
        user_id INTEGER REFERENCES twitch_user (id),
        responder_id INTEGER REFERENCES twitch_bot_responders (id) NOT NULL,
        responder_profile INTEGER REFERENCES twitch_bot_auto_response_profiles (id) NOT NULL DEFAULT 1,
        active BOOLEAN NOT NULL DEFAULT TRUE,
        last_instance INTEGER NOT NULL DEFAULT 0, -- UTC timestamp
        cooldown INTEGER NOT NULL DEFAULT 0,
        per_user_cooldown INTEGER NOT NULL DEFAULT 10,
        PRIMARY KEY (user_id, responder_id)
    );

INSERT INTO
    user_selected_responders (user_id, responder_id)
VALUES
    (167591621, 1),
    (167591621, 2),
    (167591621, 3),
    (167591621, 4);

INSERT INTO
    user_selected_responders (user_id, responder_id, responder_profile, cooldown)
VALUES
    (167591621, 5, 2, 60);