-- Your SQL goes here


CREATE TABLE twitch_user (
    id INT PRIMARY KEY,
    login TEXT NOT NULL,
    login_state TEXT REFERENCES twitch_login_process(state)
);