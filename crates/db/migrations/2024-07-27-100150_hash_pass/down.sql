-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "hash_password";
ALTER TABLE "users" ADD COLUMN "password" VARCHAR(255) NOT NULL;

