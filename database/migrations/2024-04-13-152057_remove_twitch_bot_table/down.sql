-- This file should undo anything in `up.sql`
CREATE TABLE "twitch_bot"(
	"state" TEXT NOT NULL PRIMARY KEY,
	"id" INT4,
	"login" TEXT,
	"channel_id" INT4 NOT NULL,
	FOREIGN KEY ("state") REFERENCES "twitch_login_process"("state")
);




