-- Your SQL goes here
PRAGMA foreign_keys=off;

CREATE TABLE "new_twitch_bot"(
    "state" TEXT PRIMARY KEY,
    "id" INTEGER,
    "login" TEXT,
    "channel_id" INTEGER NOT NULL,
    FOREIGN KEY ("channel_id") REFERENCES "twitch_user"("id"),
    FOREIGN KEY ("state") REFERENCES "twitch_login_process"("state")
);

INSERT INTO "new_twitch_bot"("state", "id", "login", "channel_id")
SELECT "state", "id", "login", "channel_id" FROM "twitch_bot";

DROP TABLE "twitch_bot";

ALTER TABLE "new_twitch_bot" RENAME TO "twitch_bot";

PRAGMA foreign_keys=on;
