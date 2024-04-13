-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS "twitch_bot";
CREATE TABLE "twitch_bot"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"login" TEXT NOT NULL,
	"login_state" TEXT,
	"channel_id" INT4,
	FOREIGN KEY ("channel_id") REFERENCES "twitch_user"("id")
);




