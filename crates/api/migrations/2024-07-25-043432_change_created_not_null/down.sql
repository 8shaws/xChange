-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "created_at";
ALTER TABLE "users" DROP COLUMN "updated_at";
ALTER TABLE "users" ADD COLUMN "created_at" TIMESTAMP;
ALTER TABLE "users" ADD COLUMN "updated_at" TIMESTAMP;

