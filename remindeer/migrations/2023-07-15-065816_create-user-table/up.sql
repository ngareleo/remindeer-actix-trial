CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    "id" UUID DEFAULT uuid_generate_v4() NOT NULL,
    "name" varchar(255) NOT NULL,
    "username" varchar(255) NOT NULL,
    "photo" varchar(255) NOT NULL DEFAULT 'default.png',
    "email" varchar(255) NOT NULL,
    "password" varchar(255) NOT NULL,
    "phone_number" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);