-- Your SQL goes here

ALTER TABLE "twitch_login_process" ADD COLUMN "refresh_token" TEXT;
ALTER TABLE "twitch_login_process" ADD COLUMN "token_expiry" INTEGER;
ALTER TABLE "twitch_login_process" ADD COLUMN "access_token" TEXT;
ALTER TABLE "twitch_login_process" ADD COLUMN "token_type" TEXT;
