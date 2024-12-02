-- Your SQL goes here

ALTER TABLE "twitch_login_process" DROP COLUMN "token_expiry";
ALTER TABLE "twitch_login_process" ADD COLUMN "token_expiry" INTEGER;
