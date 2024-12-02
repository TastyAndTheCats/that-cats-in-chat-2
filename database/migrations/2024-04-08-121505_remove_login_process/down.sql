-- This file should undo anything in `up.sql`

CREATE TABLE "twitch_login_process"(
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"state" TEXT NOT NULL,
	"scope" TEXT NOT NULL,
	"is_bot" BOOLEAN NOT NULL DEFAULT FALSE,
	"is_broadcaster" BOOLEAN NOT NULL DEFAULT FALSE,
	"initiated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
