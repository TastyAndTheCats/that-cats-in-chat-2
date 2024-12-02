-- Your SQL goes here
DROP TABLE IF EXISTS "twitch_bot";
CREATE TABLE "twitch_bot"(
    "state" TEXT PRIMARY KEY,
	"id" INTEGER NOT NULL,
	"login" TEXT NOT NULL,
	"login_state" TEXT,
	"channel_id" INTEGER,
	FOREIGN KEY ("channel_id") REFERENCES "twitch_user"("id"),
    FOREIGN KEY ("state") REFERENCES "twitch_login_process"("state")
);
