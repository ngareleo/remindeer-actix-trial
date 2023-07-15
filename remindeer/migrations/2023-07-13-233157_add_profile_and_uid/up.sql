CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE users ADD unid UUID NOT NULL DEFAULT (uuid_generate_v4());

ALTER TABLE users ADD photo VARCHAR(100) NOT NULL DEFAULT 'default.png';
