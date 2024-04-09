-- Your SQL goes here

ALTER TABLE "twitch_login_process" ADD COLUMN "refresh_token" TEXT;
ALTER TABLE "twitch_login_process" ADD COLUMN "token_expiry" INT4;
ALTER TABLE "twitch_login_process" ADD COLUMN "access_token" TEXT;
ALTER TABLE "twitch_login_process" ADD COLUMN "token_type" TEXT;

