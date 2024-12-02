-- Your SQL goes here

CREATE TABLE "twitch_login_process"(
	"state" TEXT NOT NULL PRIMARY KEY,
	"scope" TEXT NOT NULL,
	"code" TEXT,
	"is_bot" BOOLEAN NOT NULL,
	"is_broadcaster" BOOLEAN NOT NULL,
	"initiated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
