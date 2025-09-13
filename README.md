# 🗳️ Voting DApp (CosmWasm + Rust + Vue 3)

Aplikasi voting terdesentralisasi full-stack:
- **Frontend**: Vue 3 + Vite (UI publik & admin, connect wallet, vote TX)
- **Backend**: Rust (Axum + MySQL + Redis) untuk API, autentikasi admin, cache hasil
- **On-chain**: CosmWasm smart contract (Rust) untuk logika voting
- **Off-chain**: Indexer & Notifier (opsional) untuk sinkronisasi hasil & reminder

# System Architecture

arsitektur sistem **Voting DApp** yang terdiri dari Frontend (Vue), Backend (Axum/Rust), Smart Contract (CosmWasm), serta komponen Off-chain (Indexer/Notifier) dengan **MySQL** sebagai database utama dan Redis untuk cache.



    +-----------------------+
    |      Frontend Vue     |
    | (UI publik + admin)    |
    +-----------+-----------+
                |
                | REST API (polls, vote result, admin operations)
                v
    +-----------------------+
    |   Backend Axum        |
    | MySQL + Redis + JWT   |
    +-----------+-----------+
                |
                | Admin push poll → on-chain message
                v
    +-----------------------+
    | CosmWasm Contract     |
    | (voting-cw20)         |
    +-----------+-----------+
                |
                | Emits Vote events
                v
    +-----------------------+
    | Off-chain Indexer     |
    | reads Vote events,     |
    | writes to MySQL       |
    +-----------+-----------+
                |
                | Cached & fast queries
                v
    +-----------------------+
    | MySQL Database        |
    +-----------------------+


## ✨ Fitur
- Admin:
  - Buat, edit, publish poll
  - Push poll ke smart contract
  - Lihat status on-chain & Tx Hash
- Voter:
  - Connect wallet (Keplr/Leap)
  - Vote dengan **native denom** atau **CW20 token**
  - Tracking hasil real-time di UI
- Off-chain (opsional):
  - Indexer event `Vote` → MySQL cache
  - Notifikasi H-1/H-0 lewat email/Telegram/WhatsApp

---

## 📂 Struktur Direktori
```

voting-dapp/
├─ frontend/         # Vue 3 + Vite (UI, wallet, voting)
├─ backend/          # Axum + MySQL + Redis (API & cache)
├─ onchain/          # CosmWasm contract (voting-cw20)
├─ offchain/         # Indexer & Notifier (opsional)
├─ config/           # env untuk sandbox/production
├─ scripts/          # helper seed/deploy/cache
├─ deploy/           # TS deploy scripts (upload, instantiate, set\_env)
└─ docs/             # dokumentasi arsitektur & data model

````

---

## ⚙️ Persiapan

### Prasyarat
- [Rust](https://www.rust-lang.org/) (backend & contract)
- [Node.js](https://nodejs.org/) (frontend & deploy scripts)
- [Docker](https://docs.docker.com/) (MySQL, Redis, optimizer, indexer)
- Wallet: [Keplr](https://www.keplr.app/) atau [Leap](https://www.leapwallet.io/)

### Clone repo
```bash
git clone https://github.com/username/voting-dapp.git
cd voting-dapp
````

### Konfigurasi environment

Copy file env sesuai target:

```bash
cp config/sandbox/* .
# edit chain.env, contracts.env, backend.env, frontend.env
```

Contoh `frontend/.env.sandbox`:

```env
VITE_API_BASE=http://localhost:8080
VITE_CHAIN_ID=pion-1
VITE_RPC=https://rpc.testnet.cosmos
VITE_REST=https://api.testnet.cosmos
VITE_GAS_PRICE=0.0025untrn
VITE_VOTING_ADDR=cosmwasm1contract...
VITE_CW20_ADDR=cosmwasm1cw20...
VITE_EXPLORER_TX=https://www.mintscan.io/juno-testnet/txs
```

---

## 🚀 Jalankan Aplikasi

### 1) Jalankan services dengan Docker

```bash
docker compose up -d mysql redis
```

### 2) Backend (Axum + MySQL)

```bash
cd backend
cargo run
```

### 3) Frontend (Vue 3 + Vite)

```bash
cd frontend
npm install
npm run dev
```

### 4) Build & Deploy Contract

```bash
cd onchain
make optimize   # hasil .wasm ada di artifacts/
cd ../deploy/ts
npm install
npx ts-node scripts/upload.ts
npx ts-node scripts/instantiate.ts
```

Alamat kontrak akan otomatis ditulis ke `.env` (via `set_env_from_deploy.ts`).

---

## 🔄 Alur Voting

1. Admin login → buat poll (backend `POST /admin/polls`)
2. Admin push on-chain (backend `POST /admin/polls/:id/push-onchain`)
3. Voter buka UI → connect wallet (Keplr/Leap)
4. Klik **Vote** → TX dikirim ke contract:

   * Native token: `execute({ vote: { slug, option } }, fee, [coin])`
   * CW20 token: `cw20.send(contract, amount, hook={ vote: { ... } })`
5. Kontrak `voting-cw20` simpan suara + emit event
6. Indexer (opsional) tangkap event → update MySQL cache
7. UI refresh hasil via backend `/public/results/:slug` atau query langsung ke contract

---

## 📊 Dokumentasi

* [docs/system-architecture.md](docs/system-architecture.md) → arsitektur sistem
* [docs/sequence-flows.md](docs/sequence-flows.md) → flow FE/BE/On-chain
* [docs/data-model.md](docs/data-model.md) → skema MySQL
* [docs/api-contracts.md](docs/api-contracts.md) → daftar endpoint backend

---

## 🛠️ Pengembangan

* Contract: `cargo test` di `onchain/contracts/voting-cw20/`
* Backend: `cargo run -- --env .env.sandbox`
* Frontend: `npm run dev`
* Indexer/Notifier: build dengan `cargo run` di folder masing-masing

---
👤 Author

Kukuh Tripamungkas Wicaksono

💼 LinkedIn

📧 Email: kukuhtw@gmail.com

💬 WhatsApp: wa.me/628129893706

## 📜 Lisensi

[MIT](LICENSE) © 2025 Kukuh TW

