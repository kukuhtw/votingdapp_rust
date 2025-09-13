

### 📄 `docs/system-architecture.md`

```markdown
# System Architecture

```

\[Frontend Vue]  <--->  \[Backend Axum + MySQL + Redis]  <--->  \[CosmWasm Contract]
|
v
\[Offchain Indexer]
|
v
\[MySQL DB]

```

- **Frontend (Vue 3 + Vite)**  
  - Public: lihat daftar poll, detail, vote.  
  - Admin: login, buat/edit poll, push ke on-chain.  
- **Backend (Rust Axum)**  
  - REST API: `/public/*`, `/admin/*`  
  - Simpan metadata poll di **MySQL**  
  - Cache sementara di Redis  
- **On-chain (CosmWasm)**  
  - Smart contract voting-cw20  
  - Menyimpan hasil vote final  
- **Off-chain Indexer (Rust worker)**  
  - Stream event `Vote` via RPC/WS  
  - Sinkronkan hasil ke **MySQL** (bukan MongoDB)  
  - Percepat query hasil untuk frontend
```

---

### 📄 `docs/sequence-flows.md`

```markdown
# Sequence Flows

## 1. Admin Publish Poll
Admin UI → Backend API → MySQL → Push to Chain → Contract deployed

## 2. Voter Participate
Voter UI → Wallet (Keplr/Leap) → TX ke Contract → Contract simpan vote

## 3. Off-chain Indexer
Contract emit event `Vote` → Indexer listen → Insert hasil ke MySQL → Cache di Redis

## 4. Frontend Query Result
Frontend → Backend API → MySQL/Redis → tampilkan hasil real-time
```

---

### 📄 `docs/data-model.md`

```markdown
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
```

---

### 📄 `docs/api-contracts.md`

```markdown
# API Contracts (Backend Axum + MySQL)

## Public Endpoints
- `GET /public/polls` → daftar poll aktif
- `GET /public/polls/:slug` → detail poll
- `GET /public/results/:slug` → hasil vote (dari MySQL cache)

## Admin Endpoints
- `POST /auth/login`
- `POST /auth/register`
- `GET /admin/polls`
- `POST /admin/polls`
- `GET /admin/polls/:id`
- `PATCH /admin/polls/:id`
- `POST /admin/polls/:id/push-onchain`

## Notes
- Semua data tersimpan di **MySQL**  
- Redis dipakai untuk cache jangka pendek  
- Off-chain indexer update `votes_idx` dan `results_cache` di MySQL
```

---

### 📄 `docs/ops-runbook.md`

````markdown
# Ops Runbook

## Start services
```bash
docker compose up -d mysql redis
cargo run --manifest-path backend/Cargo.toml
npm run dev --prefix frontend
````

## Reset database

```bash
mysql -u root -p voting_dapp < migrations/schema.sql
```

## Rebuild indexer

```bash
cargo run --manifest-path offchain/indexer/Cargo.toml
```

## Backup strategy

* MySQL: dump harian
* Redis: optional snapshot
* Contracts: verify di explorer

