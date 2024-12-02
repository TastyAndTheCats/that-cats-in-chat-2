-- Your SQL goes here

CREATE TABLE "twitch_bot"(
	"state" TEXT NOT NULL PRIMARY KEY,
	"id" INTEGER,
	"login" TEXT,
	"channel_id" INTEGER NOT NULL
);
