-- Definitions of modules
CREATE TABLE twitch_bot_responder_groups (
    id SERIAL PRIMARY KEY,
    title VARCHAR(1000) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    parent INTEGER,
    
    FOREIGN KEY (parent) REFERENCES twitch_bot_responder_groups(id),
    UNIQUE (title, parent)
);

INSERT INTO twitch_bot_responder_groups (id, title) VALUES  (1, 'Core Functions'),
                                                            (2, 'Game-Related'),
                                                            (3, 'Third-Party'); 
INSERT INTO twitch_bot_responder_groups (id, title, parent) VALUES  (4, 'API Consumers', 3),
                                                                    (5, 'User-Defined', 3);

-- Definitions of responders
CREATE TABLE twitch_bot_responders (
    id SERIAL PRIMARY KEY,
    responder_group_id INTEGER REFERENCES twitch_bot_responder_groups(id),
    title VARCHAR(1000) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,

    starts_with VARCHAR(500),
    ends_with VARCHAR(500),
    contains VARCHAR(500),
    CONSTRAINT must_have_trigger CHECK (
        starts_with IS NOT NULL OR 
        ends_with IS NOT NULL OR 
        contains IS NOT NULL
    ),
    response VARCHAR(500) NOT NULL,

    requires_broadcaster BOOLEAN NOT NULL DEFAULT false,
    requires_moderator BOOLEAN NOT NULL DEFAULT false,
    requires_vip BOOLEAN NOT NULL DEFAULT false,
    requires_subscriber BOOLEAN NOT NULL DEFAULT false,
    requires_follower BOOLEAN NOT NULL DEFAULT false,

    UNIQUE (responder_group_id, title)
);

INSERT INTO twitch_bot_responders (responder_group_id, title, starts_with, response, requires_broadcaster) 
VALUES  (1, 'Say Hello', 'hello!', 'HeyGuys', true),
        (1, '!test Command', '!test', 'TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE', true);

-- Allow users to turn on and off whole modules
CREATE TABLE user_selected_modules (
    user_id INTEGER REFERENCES twitch_user(id),
    responder_group_id INTEGER REFERENCES twitch_bot_responder_groups(id) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,

    PRIMARY KEY (user_id, responder_group_id)
);

INSERT INTO user_selected_modules (user_id, responder_group_id) 
VALUES (167591621, 1);

-- Allow users to turn on and off specific responders in a module
CREATE TABLE user_selected_responders (
    user_id INTEGER REFERENCES twitch_user(id),
    responder_id INTEGER REFERENCES twitch_bot_responders(id) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,

    PRIMARY KEY (user_id, responder_id)
);

INSERT INTO user_selected_responders (user_id, responder_id) 
VALUES  (167591621, 1),
        (167591621, 2);