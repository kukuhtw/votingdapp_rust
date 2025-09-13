# Data Model (MySQL)

## Table: polls
- id (UUID, PK)
- slug (VARCHAR)
- title (TEXT)
- description (TEXT)
- options_json (JSON)
- start (BIGINT, nullable)
- end (BIGINT)
- vote_price (VARCHAR)
- status (ENUM: draft/published/closed)
- onchain_status (ENUM: none/pending/success/failed)
- onchain_tx_hash (VARCHAR, nullable)
- created_at, updated_at (TIMESTAMP)

## Table: votes_idx
- id (AUTO, PK)
- poll_id (UUID, FK polls.id)
- voter (VARCHAR)
- option (VARCHAR)
- amount (DECIMAL)
- tx_hash (VARCHAR)
- block_height (BIGINT)
- created_at (TIMESTAMP)

## Table: results_cache
- poll_id (UUID, FK polls.id)
- option (VARCHAR)
- total_votes (DECIMAL)
- updated_at (TIMESTAMP)

## Table: admin_users
- id (UUID, PK)
- email (VARCHAR UNIQUE)
- password_hash (TEXT)
- created_at (TIMESTAMP)
