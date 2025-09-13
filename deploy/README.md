
# Deploy Guide

## Sandbox
1. Isi `env.sandbox.toml` dengan mnemonic testnet & CW20.
2. Jalankan:
```bash
node ts/scripts/upload.ts --env sandbox
node ts/scripts/instantiate.ts --env sandbox
node ts/scripts/set_env_from_deploy.ts --env sandbox
```

## Production
- Gunakan Ledger/Multisig.
- Jangan simpan mnemonic di repo.
- Isi `env.production.toml`.

## Verify
```bash
node ts/scripts/verify.ts --env sandbox
```
