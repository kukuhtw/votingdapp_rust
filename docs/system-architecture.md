
# System Architecture

```mermaid
flowchart TD
  User[User Voter] --> FE[Frontend Vue]
  Admin[Admin] --> FE
  FE --> BE[Backend Rust Axum]
  BE <--> Mongo[(MongoDB)]
  BE <--> Redis[(Redis)]
  BE <--> Chain[Cosmos Chain]

  subgraph Off-chain
    IDX[Index Worker]
    NOTIF[Notifier]
    KEEP[Keeper]
  end

  Chain --> IDX
  IDX --> Mongo
  NOTIF --> BE
  KEEP --> Chain
```

- **Frontend**: Vue SPA, connect Keplr wallet.
- **Backend**: Axum API, persist data in Mongo, cache in Redis.
- **On-chain**: CosmWasm voting contract.
- **Off-chain**: 
  - Indexer tail event `Vote`.
  - Notifier kirim reminder.
  - Keeper eksekusi ClosePoll.
