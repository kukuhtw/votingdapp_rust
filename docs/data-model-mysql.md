erDiagram
  ADMIN_USERS ||--o{ POLLS : "created_by (opsional)"
  POLLS ||--o{ VOTES_IDX : "poll_id"
  POLLS ||--o{ RESULTS_CACHE : "poll_id"

  ADMIN_USERS {
    char(36) id PK
    varchar email UK
    text password_hash
    bigint created_at
  }

  POLLS {
    char(36) id PK
    varchar slug UK
    varchar title
    text description
    json options_json
    bigint start "nullable (unix ts)"
    bigint end "unix ts"
    varchar vote_price "contoh: 1000ujuno / 1000untrn"
    enum status "draft|published|closed"
    enum onchain_status "none|pending|success|failed"
    varchar onchain_tx_hash "nullable"
    bigint onchain_at "nullable"
    bigint created_at
    bigint updated_at
  }

  VOTES_IDX {
    bigint id PK
    char(36) poll_id FK
    varchar voter "bech32 address"
    int option_idx
    decimal amount ">= 0"
    varchar denom "ujuno|untrn|cw20"
    varchar tx_hash
    bigint block_height
    bigint created_at
  }

  RESULTS_CACHE {
    char(36) poll_id FK
    int option_idx
    decimal total_amount "sum by option"
    bigint updated_at
  }
