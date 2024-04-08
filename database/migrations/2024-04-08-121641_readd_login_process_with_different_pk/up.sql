-- Your SQL goes here

CREATE TABLE "twitch_login_process"(
	"state" TEXT NOT NULL PRIMARY KEY,
	"scope" TEXT NOT NULL,
	"code" TEXT,
	"is_bot" BOOL NOT NULL,
	"is_broadcaster" BOOL NOT NULL,
	"initiated_at" TIMESTAMP NOT NULL
);

