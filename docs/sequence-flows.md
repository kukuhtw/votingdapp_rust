# Sequence Flows

## 1. Admin Publish Poll
Admin UI → Backend API → MySQL → Push to Chain → Contract deployed

## 2. Voter Participate
Voter UI → Wallet (Keplr/Leap) → TX ke Contract → Contract simpan vote

## 3. Off-chain Indexer
Contract emit event `Vote` → Indexer listen → Insert hasil ke MySQL → Cache di Redis

## 4. Frontend Query Result
Frontend → Backend API → MySQL/Redis → tampilkan hasil real-time
