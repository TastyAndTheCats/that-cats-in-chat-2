-- Your SQL goes here

DROP TABLE IF EXISTS "twitch_channel";
CREATE TABLE "twitch_channel"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"login" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"language" TEXT NOT NULL,
	"delay" INTEGER NOT NULL
);

DROP TABLE IF EXISTS "twitch_games";
