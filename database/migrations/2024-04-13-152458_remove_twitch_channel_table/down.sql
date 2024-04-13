-- This file should undo anything in `up.sql`

CREATE TABLE "twitch_channel"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"login" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"language" TEXT NOT NULL,
	"delay" INT4 NOT NULL
);



