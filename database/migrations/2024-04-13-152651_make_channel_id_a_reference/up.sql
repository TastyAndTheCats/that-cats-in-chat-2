-- Your SQL goes here
ALTER TABLE "twitch_bot" DROP COLUMN "channel_id";
ALTER TABLE "twitch_bot" ADD COLUMN "channel_id" INT4 NOT NULL REFERENCES "twitch_user"("id");



