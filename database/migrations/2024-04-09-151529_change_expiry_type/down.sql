-- This file should undo anything in `up.sql`

ALTER TABLE "twitch_login_process" DROP COLUMN "token_expiry";
ALTER TABLE "twitch_login_process" ADD COLUMN "token_expiry" INTEGER;
