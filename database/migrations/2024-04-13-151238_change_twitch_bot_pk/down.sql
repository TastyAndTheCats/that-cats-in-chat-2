-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS "twitch_bot";
CREATE TABLE "twitch_bot"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"login" TEXT NOT NULL,
	"login_state" TEXT,
	"channel_id" INTEGER,
	FOREIGN KEY ("channel_id") REFERENCES "twitch_user"("id")
);
