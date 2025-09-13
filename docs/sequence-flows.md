
# Sequence Flows

## Voting Flow
1. User buka FE → fetch poll list dari BE.
2. User connect wallet (Keplr).
3. User pilih opsi → TX CW20::Send + hook Vote → on-chain contract.
4. Kontrak validasi:
   - poll aktif & belum expired.
   - user belum pernah vote.
   - cukup balance & amount.
5. Event `Vote` emited.
6. Indexer baca event → tulis ke Mongo.votes_idx.
7. BE expose `/public/polls/:id/result` pakai cache Redis.

## Admin Publish Flow
1. Admin login (JWT).
2. Admin create draft poll → save ke Mongo (status=draft).
3. Admin tekan "Publish" → FE panggil BE.
4. BE instantiate poll di kontrak on-chain.
5. Simpan txhash & update status=pending/published.
