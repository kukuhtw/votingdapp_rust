# System Architecture

Dokumen ini menjelaskan arsitektur sistem **Voting DApp** yang terdiri dari Frontend (Vue), Backend (Axum/Rust), Smart Contract (CosmWasm), serta komponen Off-chain (Indexer/Notifier) dengan **MySQL** sebagai database utama dan Redis untuk cache.

## High-level Diagram (Mermaid)

## High-level Diagram (Mermaid)

```mermaid
sequenceDiagram
  participant V as Voter (Frontend)
  participant A as Backend API
  participant C as Contract (voting-cw20)
  participant X as Indexer
  participant M as MySQL
  participant R as Redis

  V->>A: GET /public/polls (list/detail)
  A->>R: check cache
  alt cache hit
    R-->>A: cached result
  else cache miss
    A->>M: query polls/results
    M-->>A: rows
    A->>R: set cache
  end
  A-->>V: JSON

  Note over V,C: Voting tx (Keplr/Leap)
  V->>C: execute vote (native or CW20 hook)
  C-->>V: tx hash

  C-->>X: emit Vote event
  X->>M: upsert votes_idx / results_cache

  V->>A: GET /public/results/:slug
  A->>R: check cache
  R-->>A: cached (or A->>M if miss)
  A-->>V: aggregated results


## Komponen

| Komponen            | Fungsi                                                                 |
|---------------------|------------------------------------------------------------------------|
| **Frontend Vue**     | UI untuk publik dan admin: tampil poll, vote, login admin, push on-chain |
| **Backend Axum**     | REST endpoints, otentikasi, simpan metadata poll di MySQL, caching via Redis |
| **CosmWasm Contract**| Logika voting: menerima vote, validasi, menyimpan ke state blockchain     |
| **Off-chain Indexer** | Mendengarkan event dari contract → simpan hasil vote ke MySQL untuk performa |
| **MySQL**            | Menyimpan semua data: polls, hasil voting, metadata, cache               |
| **Redis**            | Cache sementara untuk mempercepat read (mis. hasil polling)             |

---

## Alur Data & Interaksi

1. **Admin membuat poll** di frontend → dikirim ke backend → disimpan ke tabel `polls` di MySQL (status: draft).  
2. **Admin push on-chain** → backend memanggil fungsi ke contract via gRPC/execute → contract di-deploy/instantiate.  
3. **Voter vote** via frontend + wallet (native atau CW20) → TX ke contract.  
4. Contract emit event `Vote` → off-chain indexer tangkap event → tulis ke tabel `votes_idx` di MySQL.  
5. Backend membaca cached hasil dari MySQL + Redis → frontend publik dapat tampil cepat hasil polling.  

---

## Keuntungan Arsitektur Ini

- MySQL sebagai **single source of truth** untuk data polling & hasil → memudahkan query / storage  
- Off-chain indexer** mempercepat dan mengurangi dependensi frontend langsung ke smart contract untuk hasil  
- Redis untuk cache → mengurangi latensi baca di frontend  
- Arsitetur modular: bisa deploy backend atau indexer terpisah, ubah chain RPC, dsb.

