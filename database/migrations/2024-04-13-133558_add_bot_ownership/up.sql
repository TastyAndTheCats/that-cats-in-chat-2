-- Your SQL goes here

ALTER TABLE "twitch_bot" ADD COLUMN "channel_id" INTEGER REFERENCES twitch_user(id);
