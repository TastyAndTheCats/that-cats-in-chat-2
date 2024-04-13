-- This file should undo anything in `up.sql`
ALTER TABLE "twitch_bot" DROP COLUMN "id";
ALTER TABLE "twitch_bot" DROP COLUMN "login";
ALTER TABLE "twitch_bot" DROP COLUMN "channel_id";
ALTER TABLE "twitch_bot" ADD COLUMN "login_state" TEXT;
ALTER TABLE "twitch_bot" ADD COLUMN "id" INT4 NOT NULL;
ALTER TABLE "twitch_bot" ADD COLUMN "login" TEXT NOT NULL;
ALTER TABLE "twitch_bot" ADD COLUMN "channel_id" INT4;




