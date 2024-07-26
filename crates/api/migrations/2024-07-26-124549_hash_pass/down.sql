-- This file should undo anything in `up.sql`
ALTER TABLE "users" ADD COLUMN "salt" VARCHAR(255) NOT NULL;

