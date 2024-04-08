-- This file should undo anything in `up.sql`

CREATE TABLE "twitch_login_process"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"state" TEXT NOT NULL,
	"scope" TEXT NOT NULL,
	"is_bot" BOOL NOT NULL,
	"is_broadcaster" BOOL NOT NULL,
	"initiated_at" TIMESTAMP NOT NULL
);

