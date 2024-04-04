-- Your SQL goes here
CREATE TABLE "twitch_channel"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"login" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"language" TEXT NOT NULL,
	"game_id" TEXT NOT NULL,
	"title" TEXT NOT NULL,
	"delay" INT NOT NULL,
	"content_classification" TEXT NOT NULL,
	"is_branded_content" BOOLEAN NOT NULL
);

CREATE TABLE "twitch_games"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL
);

