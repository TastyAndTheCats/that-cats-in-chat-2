-- This file should undo anything in `up.sql`

ALTER TABLE "twitch_login_process" DROP COLUMN "refresh_token";
ALTER TABLE "twitch_login_process" DROP COLUMN "token_expiry";
ALTER TABLE "twitch_login_process" DROP COLUMN "access_token";
ALTER TABLE "twitch_login_process" DROP COLUMN "token_type";

