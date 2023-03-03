-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS feedbacks (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        rating INTEGER NOT NULL,
        text TEXT NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );