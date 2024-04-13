-- Your SQL goes here
ALTER TABLE "twitch_bot" DROP COLUMN "login_state";
ALTER TABLE "twitch_bot" DROP COLUMN "id";
ALTER TABLE "twitch_bot" DROP COLUMN "login";
ALTER TABLE "twitch_bot" DROP COLUMN "channel_id";
ALTER TABLE "twitch_bot" ADD COLUMN "id" INT4;
ALTER TABLE "twitch_bot" ADD COLUMN "login" TEXT;
ALTER TABLE "twitch_bot" ADD COLUMN "channel_id" INT4 NOT NULL;




