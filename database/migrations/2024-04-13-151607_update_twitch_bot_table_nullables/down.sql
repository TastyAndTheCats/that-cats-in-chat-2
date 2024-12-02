-- This file should undo anything in `up.sql`
PRAGMA foreign_keys=off;

CREATE TABLE "new_twitch_bot"(
    "state" TEXT PRIMARY KEY,
    "id" INTEGER NOT NULL,
    "login" TEXT NOT NULL,
    "login_state" TEXT,
    "channel_id" INTEGER,
    FOREIGN KEY ("channel_id") REFERENCES "twitch_user"("id"),
    FOREIGN KEY ("state") REFERENCES "twitch_login_process"("state")
);

INSERT INTO "new_twitch_bot"("state", "id", "login", "login_state", "channel_id")
SELECT "state", "id", "login", "login_state", "channel_id" FROM "twitch_bot";

DROP TABLE "twitch_bot";

ALTER TABLE "new_twitch_bot" RENAME TO "twitch_bot";

PRAGMA foreign_keys=on;
