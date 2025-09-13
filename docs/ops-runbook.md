Oke 👍 mari kita perbaiki `docs/ops-runbook.md` supaya konsisten dengan MySQL (bukan Mongo), tambahkan langkah yang lebih lengkap, plus tambahkan **diagram Mermaid** untuk memperjelas alur ops.

---

# Ops Runbook

## Start/Stop Services

```bash
# Jalankan semua service
docker compose up -d

# Hentikan semua service
docker compose down
```

## Logs

```bash
# Lihat log backend
docker compose logs -f backend

# Lihat log indexer (off-chain worker)
docker compose logs -f indexer

# Lihat log frontend (opsional)
docker compose logs -f frontend
```

## Database & Cache Backup

* **MySQL**: lakukan dump rutin (harian/mingguan)

```bash
mysqldump -h127.0.0.1 -uvoting -pstrongpass voting_dapp_prod > backup/voting_$(date +%F).sql
```

* **Redis**: snapshot RDB otomatis. Untuk backup manual:

```bash
redis-cli SAVE
cp dump.rdb backup/redis_$(date +%F).rdb
```

## Key & Secret Rotation

* **JWT\_SECRET**: ganti di `backend.env` → restart backend.
* **Cosmos Mnemonic**: hanya disimpan di `deploy/keys/` (gitignored). Pastikan permission terbatas.
* **DB Password**: update di `.env` & `docker-compose.yml`, lalu restart service MySQL & backend.

## Incident Handling

* **Indexer mati**
  → restart container:

  ```bash
  docker compose restart indexer
  ```

* **Kontrak mismatch (alamat/versi)**
  → redeploy kontrak pakai `make deploy` → update `contracts.env` (frontend & backend) → restart service.

* **MySQL crash**
  → restore dari backup terbaru:

  ```bash
  mysql -uvoting -pstrongpass voting_dapp_prod < backup/voting_2025-09-13.sql
  ```

---

## Ops Architecture (Mermaid)

```mermaid
flowchart TD
    subgraph USER[Operator / DevOps]
      CMD[Docker Compose CLI]
      BAK[Backup Scripts]
    end

    subgraph STACK[Runtime Stack]
      FE[Frontend (Vue)]
      BE[Backend (Axum)]
      IDX[Off-chain Indexer]
      DB[(MySQL)]
      REDIS[(Redis)]
      CHAIN[CosmWasm Contract]
    end

    CMD --> FE
    CMD --> BE
    CMD --> IDX
    BE --> DB
    BE --> REDIS
    IDX --> DB
    FE --> BE
    BE --> CHAIN
    FE --> CHAIN
    BAK --> DB
    BAK --> REDIS
```

---

👉 Dengan runbook ini:

* Start/stop service jelas.
* Backup & restore pakai MySQL (bukan Mongo lagi).
* Ada langkah key rotation & incident handling.
* Mermaid diagram membantu operator memahami hubungan antar service.
