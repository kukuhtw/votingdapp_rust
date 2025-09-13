
# Ops Runbook

## Start/Stop
```bash
docker compose up -d
docker compose down
```

## Logs
```bash
docker compose logs -f backend
docker compose logs -f indexer
```

## Backup
- Mongo: `mongodump --uri=$MONGO_URI -o backup/`
- Redis: RDB snapshot otomatis.

## Key Rotation
- JWT_SECRET → ganti & restart backend.
- Mnemonic deploy hanya di `deploy/keys/` (gitignored).

## Incident
- Jika indexer mati → restart container.
- Jika kontrak mismatch → redeploy & update `contracts.env`.
