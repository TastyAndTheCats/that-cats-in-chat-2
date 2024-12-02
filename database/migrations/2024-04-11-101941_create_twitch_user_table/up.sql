-- Your SQL goes here

CREATE TABLE twitch_user (
    id INTEGER PRIMARY KEY,
    login TEXT NOT NULL,
    login_state TEXT REFERENCES twitch_login_process(state)
);
