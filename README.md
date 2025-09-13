
# 🗳️ Voting DApp – Cosmos + CosmWasm + Rust + Vue

Aplikasi voting berbasis blockchain:
- **On-chain**: CosmWasm kontrak voting (bayar dengan CW20 stablecoin, 1 user 1 vote per konten, expiry).
- **Backend**: Rust (Axum) + MongoDB + Redis
- **Frontend**: Vue 3 + Vite
- **Off-chain**: Indexer & Notifier (Rust tokio)
- **Deploy**: CosmJS script (TypeScript)

---

## 🚀 Quick Start (Sandbox)

```bash
# copy env sandbox
make env-sandbox

# run stack
docker compose up -d --build

# backend: http://localhost:8081
# frontend: http://localhost:5173
```

---

## 📦 Struktur Proyek
- `frontend/` – UI Vue 3
- `backend/` – API Rust (Axum)
- `offchain/` – indexer/notifier/keeper
- `onchain/` – kontrak CosmWasm
- `deploy/` – script deploy & config
- `docs/` – dokumentasi teknis

---

## ⚡ Fitur Utama
- Admin membuat konten voting dengan expiry.
- User memilih opsi dengan bayar CW20 (misal axlUSDC).
- 1 user hanya bisa 1 kali vote per konten.
- Indexer sinkron hasil → Redis cache.
- Public bisa lihat hasil secara real-time.

---

## 🔒 Keamanan
- JWT untuk admin dashboard.
- Password hash pakai Argon2.
- Env variabel terpisah sandbox/production.
- Jangan commit `deploy/keys/`.

---

## 📖 Dokumentasi
Lihat folder [`docs/`](./docs).
