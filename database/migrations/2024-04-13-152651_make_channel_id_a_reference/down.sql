-- This file should undo anything in `up.sql`
ALTER TABLE "twitch_bot" DROP COLUMN "channel_id";
ALTER TABLE "twitch_bot" ADD COLUMN "channel_id" INT4 NOT NULL;



