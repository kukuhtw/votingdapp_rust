
# API Contracts (Backend)

## Auth
- POST `/auth/login` → {email,password} → {token}
- POST `/auth/register` (opsional)

## Admin
- GET `/admin/polls` (JWT)
- POST `/admin/polls` (JWT)
- GET `/admin/polls/:id` (JWT)
- PATCH `/admin/polls/:id` (JWT)

## Public
- GET `/public/polls`
- GET `/public/polls/:slug`
- GET `/public/polls/:slug/result`

## Cache Refresh
- POST `/admin/refresh` (JWT) → trigger refresh Redis.
