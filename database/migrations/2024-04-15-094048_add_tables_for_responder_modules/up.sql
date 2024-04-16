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
    twitch_bot_responder_groups (id, title, parent)
VALUES
    (1, 'Core Functions', NULL),
    (2, 'Game-Related', NULL),
    (3, 'Third-Party', NULL),
    (4, 'API Consumers', 3),
    (5, 'User-Defined', 3),
    (6, 'Epic Games Store', 4),
    (7, 'Facts', 1),
    (8, 'Information', 1),
    (9, 'Twitch', 4),
    (10, 'YouTube', 4),
    (11, 'Chatter Status', 1),
    (12, 'Mathematics', 1),
    (13, 'Niceties', 1);

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
        UNIQUE (responder_group_id, title)
    );

-- just response
INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, contains, ends_with, response)
VALUES
    (1, 1, 'Say Hello', 'hello!', NULL, NULL, 'HeyGuys'),
    (2, 1, '!test Command', NULL, '!test', NULL, 'TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE'),
    (24, 11, 'Lurk', NULL, '!lurk', NULL, 'have distracted {sender} and they are now playing with the kitties'),
    (
        25,
        11,
        'Unlurk',
        NULL,
        '!unlurk',
        NULL,
        'has released {sender} from the cuteness trap and they have returned to spend time in chat'
    ),
    (26, 11, 'BRB', NULL, '!brb', NULL, 'have momentarily distracted {sender}'),
    (27, 11, 'UnBRB', NULL, '!back', NULL, 'have become bored with {sender} and have allowed them to return to chat'),
    (30, 13, 'Illuminati', NULL, 'illuminati', NULL, 'TheIlluminati TheIlluminati TheIlluminati TheIlluminati TheIlluminati'),
    (
        31,
        13,
        'English Please!',
        '!english',
        NULL,
        NULL,
        'Solo ingles Gracias / ภาษาอังกฤษเท่านั้น / Sadece Ingilizce / Только по Английски! Спасибо 
        / 只有英文谢谢 / alleen Engels / אנגלית בלבד בבקשה / apenas inglês / Nur Englisch, Vielen Dank 
        / انجليزي فقط. شكرا / que l''anglais / English only Please! Thank you. 💗'
    ),
    (
        32,
        13,
        'TERRIBLE',
        NULL,
        'terrible|misery',
        NULL,
        'Remember to use command ⭐Alt+F4⭐ if you''re not enjoying the stream. 
        This will end your misery abruptly 🏳️ hope it''s working'
    ),
    (
        33,
        13,
        'Suicide Hotline',
        '!hotline', 
        NULL,
        NULL,
        'If you or someone you know is contemplating suicide, please reach out to a professional. 
        You can find help at a National Suicide Prevention Lifeline 
        => USA: 18002738255 | US Crisis textline: 741741 text HOME 
        => Crisis Services Canada: 833-456-4566 | SMS 45645 
        => United Kingdom: 116 123 
        => Trans Lifeline (877-565-8860) 
        => Others: https://en.wikipedia.org/wiki/List_of_suicide_crisis_lines https://suicidepreventionlifeline.org'
    ),
    (34, 13, 'Hello!', '!hello', NULL, NULL, 'Hello {sender}! HeyGuys'),
    (36, 13, 'Are you Muted?', NULL, '!muted', NULL, 'Hey {channel_name}, I think you''re muted! {sender} says they can''t hear you!'),
    (37, 13, 'FPS Issue', NULL, '!fps|!frames|!framedrop', NULL, 'Hey {channel_name}, you might be dropping frames - {sender} says they''re having trouble with your video feed'),
    (38, 13, 'Wrong Scene', NULL, '!muted', NULL, 'Hey {channel_name}, check your feed - {sender} says they think you''re broadcasting the wrong scene');

-- just response_fn
INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, contains, ends_with, response_fn)
VALUES
    (3, 1, '!fntest Command', NULL, '!fntest', NULL, 'unpack_the_galaxy'),
    (5, 6, 'Epic Store Free Games', '!epic|!epicfree', 'epic games store', 'epic?|epic', 'api::epic_store::free_games'),
    (6, 7, 'Dog Facts', NULL, '!dogfact', NULL, 'core::facts::dogfact'),
    (7, 7, 'Cat Facts', NULL, '!catfact', NULL, 'core::facts::catfact'),
    (8, 7, 'Number Facts', NULL, '!numfact|!numberfact', NULL, 'core::facts::dognumfactfact'),
    (9, 7, 'Chuck Norris Facts', NULL, '!chuckfact|!norrisfact|!cnfact|!chucknorris', NULL, 'core::facts::chucknorris'),
    (10, 7, 'Advice', NULL, '!advice', NULL, 'core::facts::advice'),
    (11, 7, 'Dad Joke', NULL, '!dadjoke', NULL, 'core::facts::dadjoke'),
    (12, 8, 'Time', '!time', NULL, NULL, 'core::info::time'),
    (13, 8, 'Weather', '!weather', NULL, NULL, 'core::info::weather'),
    (14, 4, 'Wikia', '!wikia', NULL, NULL, 'api::wikia::lookup'),
    (15, 4, 'Wikipedia (Long)', '!wikipedia', NULL, NULL, 'api::wikipedia::lookup_full'),
    (16, 4, 'Wikipedia (Short)', '!wiki', NULL, NULL, 'api::wikipedia::lookup'),
    (17, 8, 'Dictionary', '!dict|!dictionary|!define|!def', NULL, NULL, 'core::info::dictionary'),
    (18, 8, 'Thesaurus', '!thesaurus|!wordslike', NULL, NULL, 'core::info::thesaurus'),
    (19, 9, 'Set Game', '!setgame', NULL, NULL, 'core::twitch::set_game'),
    (20, 9, 'Set Title', '!settitle', NULL, NULL, 'core::twitch::set_title'),
    (21, 10, 'Youtube Video Info', NULL, 'youtube.com|youtu.be', NULL, 'core::youtube::get_video_info'),
    (22, 1, 'Chatbot Info', NULL, '!thecats|!thecatsinchat|!tcic', NULL, 'core::info'),
    (23, 1, 'Chatbot Commands', NULL, '!commands|!catcommands', NULL, 'core::commands'),
    (28, 12, 'Prime Number Checking', '!isprime|!primecheck', NULL, NULL, 'core::maths::prime_check'),
    (29, 12, 'Coin Flip', NULL, '!coinflip|!flipcoin|!cointoss|!tosscoin', NULL, 'core::maths::coin_toss'),
    (35, 13, 'Shoutout', NULL, '!gowatch|!gofollow|!so', NULL, 'core::niceties::shoutout');

-- response and response_fn
INSERT INTO
    twitch_bot_responders (id, responder_group_id, title, STARTS_WITH, contains, ends_with, response, response_fn)
VALUES
    (4, 1, '!bothtest Command', NULL, '!bothtest', NULL, 'This is a test of using a response and a response function', 'default');

-- Set who can and can't use each command
CREATE TABLE
    twitch_bot_responder_permissions (
        id SERIAL PRIMARY KEY,
        title VARCHAR(1000) NOT NULL,
        requires_broadcaster BOOLEAN NOT NULL DEFAULT FALSE,
        requires_moderator BOOLEAN NOT NULL DEFAULT FALSE,
        requires_vip BOOLEAN NOT NULL DEFAULT FALSE,
        requires_subscriber BOOLEAN NOT NULL DEFAULT FALSE,
        requires_follower BOOLEAN NOT NULL DEFAULT FALSE
    );

INSERT INTO
    twitch_bot_responder_permissions (id, title, requires_broadcaster, requires_moderator, requires_vip, requires_subscriber, requires_follower)
VALUES
    (1, 'Anyone', FALSE, FALSE, FALSE, FALSE, FALSE),
    (2, 'Broadcaster-Only', TRUE, FALSE, FALSE, FALSE, FALSE),
    (3, 'Broadcaster and Mod', TRUE, TRUE, FALSE, FALSE, FALSE),
    (4, 'Broadcaster, Mod, and VIP', TRUE, TRUE, TRUE, FALSE, FALSE),
    (5, 'Subscriber+', TRUE, TRUE, TRUE, TRUE, FALSE),
    (6, 'Follower+', TRUE, TRUE, TRUE, TRUE, TRUE),
    (7, 'Moderator-Only', FALSE, TRUE, FALSE, FALSE, FALSE),
    (8, 'VIP-Only', FALSE, FALSE, FALSE, TRUE, FALSE),
    (9, 'Subscriber-Only', FALSE, FALSE, FALSE, TRUE, FALSE),
    (10, 'Follower-Only', FALSE, FALSE, FALSE, FALSE, TRUE);

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
        permissions INTEGER REFERENCES twitch_bot_responder_permissions (id) NOT NULL DEFAULT 2,
        cooldown INTEGER NOT NULL DEFAULT 0,
        per_user_cooldown INTEGER NOT NULL DEFAULT 10,
        include_specific_users TEXT DEFAULT NULL,
        exclude_specific_users TEXT DEFAULT NULL,
        CONSTRAINT include_or_exclude_not_both CHECK (
            (
                include_specific_users IS NULL AND
                exclude_specific_users IS NULL
            ) OR
            (
                include_specific_users IS NULL OR
                exclude_specific_users IS NULL
            )
        ),
        PRIMARY KEY (user_id, responder_id)
    );

INSERT INTO
    user_selected_responders (user_id, responder_id)
VALUES
    (167591621, 1),
    (167591621, 2),
    (167591621, 3),
    (167591621, 4),
    -- ,
    (167591621, 6),
    (167591621, 7),
    (167591621, 8),
    (167591621, 9),
    (167591621, 10),
    (167591621, 11),
    (167591621, 12),
    (167591621, 13),
    (167591621, 14),
    (167591621, 15),
    (167591621, 16),
    (167591621, 17),
    (167591621, 18),
    (167591621, 19),
    (167591621, 20),
    (167591621, 21),
    (167591621, 22),
    -- ,
    (167591621, 24),
    (167591621, 25),
    (167591621, 26),
    (167591621, 27),
    (167591621, 28),
    (167591621, 29),
    (167591621, 30),
    (167591621, 31),
    (167591621, 32),
    -- ,
    (167591621, 34),
    (167591621, 35),
    (167591621, 36),
    (167591621, 37),
    (167591621, 38);

INSERT INTO
    user_selected_responders (user_id, responder_id, responder_profile, cooldown)
VALUES
    (167591621, 5, 2, 60),
    (167591621, 23, 3, 30);

INSERT INTO
    user_selected_responders (user_id, responder_id, permissions)
VALUES
    (167591621, 33, 4);