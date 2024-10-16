-- Migration script to add cohorts table
CREATE TABLE cohorts (
    id SERIAL PRIMARY KEY,
    address VARCHAR(45) NOT NULL,
    build VARCHAR(100) NOT NULL,
    tx_hash VARCHAR(64), 
    created_at TIMESTAMPTZ DEFAULT NOW()
);
