-- Your SQL goes here
ALTER TABLE "users" DROP COLUMN "created_at";
ALTER TABLE "users" DROP COLUMN "updated_at";
ALTER TABLE "users" ADD COLUMN "created_at" TIMESTAMP NOT NULL;
ALTER TABLE "users" ADD COLUMN "updated_at" TIMESTAMP NOT NULL;

