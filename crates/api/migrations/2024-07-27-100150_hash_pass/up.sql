-- Your SQL goes here
ALTER TABLE "users" DROP COLUMN "password";
ALTER TABLE "users" ADD COLUMN "hash_password" VARCHAR(255) NOT NULL;

