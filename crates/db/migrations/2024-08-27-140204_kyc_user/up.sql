-- Your SQL goes here
CREATE TABLE "kyc"(
	"id" UUID NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"document_type" VARCHAR(255) NOT NULL,
	"document_number" VARCHAR(255) NOT NULL,
	"issue_country" VARCHAR(255) NOT NULL,
	"expiry_date" TIMESTAMP NOT NULL,
	"document_front_url" VARCHAR(1000) NOT NULL,
	"document_back_url" VARCHAR(1000) NOT NULL,
	"selfie_url" VARCHAR(1000) NOT NULL,
	"verification_status" VARCHAR NOT NULL,
	"submitted_at" TIMESTAMP NOT NULL,
	"verified_at" TIMESTAMP,
	"rejected_at" TIMESTAMP,
	"rejection_reason" VARCHAR(1000),
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);
