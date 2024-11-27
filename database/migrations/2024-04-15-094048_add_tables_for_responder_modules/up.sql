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

-- Allow users to turn on and off whole modules
CREATE TABLE
    user_selected_modules (
        user_id INTEGER REFERENCES twitch_user (id),
        responder_group_id INTEGER REFERENCES twitch_bot_responder_groups (id) NOT NULL,
        active BOOLEAN NOT NULL DEFAULT TRUE,
        PRIMARY KEY (user_id, responder_group_id)
    );

-- Different auto-response profiles
CREATE TABLE
    twitch_bot_auto_response_profiles (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        INTERVAL INTEGER DEFAULT 3600, -- minimum seconds before this auto-response fires again
        distance INTEGER DEFAULT 50 -- minimum non-bot messages between repeat instances of this message
    );

-- Allow users to turn on and off specific responders in a module
CREATE TABLE
    user_selected_responders (
        user_id INTEGER REFERENCES twitch_user (id),
        responder_id INTEGER REFERENCES twitch_bot_responders (id) NOT NULL,
        responder_profile INTEGER REFERENCES twitch_bot_auto_response_profiles (id) NOT NULL DEFAULT 1,
        active BOOLEAN NOT NULL DEFAULT TRUE,
        last_instance INTEGER NOT NULL DEFAULT 0, -- UTC timestamp
        permissions INTEGER REFERENCES twitch_bot_responder_permissions (id) NOT NULL DEFAULT 3,
        cooldown INTEGER NOT NULL DEFAULT 10,
        per_user_cooldown INTEGER NOT NULL DEFAULT 60,
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
        count INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY (user_id, responder_id)
    );