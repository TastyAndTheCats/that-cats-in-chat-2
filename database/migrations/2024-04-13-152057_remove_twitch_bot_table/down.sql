-- This file should undo anything in `up.sql`
CREATE TABLE "twitch_bot"(
	"state" TEXT NOT NULL PRIMARY KEY,
	"id" INTEGER,
	"login" TEXT,
	"channel_id" INTEGER NOT NULL,
	FOREIGN KEY ("state") REFERENCES "twitch_login_process"("state")
);
