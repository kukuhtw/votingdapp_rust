# voting-dapp/Dockerfile
# ---- Base builder Rust (gunakan versi >=1.82 biar aman dg crates baru) ----
FROM rust:1.82-bullseye AS rust-builder
WORKDIR /app

# 1) Copy manifest workspace + member manifests (TANPA src) → cache deps
#    (tambahkan crate lain kalau ada)
COPY Cargo.toml Cargo.lock ./

# backend + offchain manifests
COPY backend/Cargo.toml backend/Cargo.toml
COPY offchain/indexer/Cargo.toml offchain/indexer/Cargo.toml
COPY offchain/notifier/Cargo.toml offchain/notifier/Cargo.toml
COPY offchain/keeper/Cargo.toml offchain/keeper/Cargo.toml

# 2) Stub minimal (dummy) untuk prebuild cache dependencies
RUN mkdir -p backend/src offchain/indexer/src offchain/notifier/src offchain/keeper/src && \
    echo "fn main(){}" > backend/src/main.rs && \
    echo "fn main(){}" > offchain/indexer/src/main.rs && \
    echo "fn main(){}" > offchain/notifier/src/main.rs && \
    echo "fn main(){}" > offchain/keeper/src/main.rs

# 3) Pre-fetch/build dependencies (boleh gagal—hanya untuk cache)
RUN cargo fetch && cargo build --release || true


# ---- Build backend binary ----
FROM rust-builder AS backend-build
WORKDIR /app
# Ganti stub dgn source asli backend
COPY backend/src backend/src
COPY backend/.env.sandbox backend/.env.sandbox
COPY backend/.env.production backend/.env.production
RUN cargo build -p backend --release


# ---- Build indexer binary ----
FROM rust-builder AS indexer-build
WORKDIR /app
COPY offchain/indexer/src offchain/indexer/src
RUN cargo build -p indexer --release


# ---- Build notifier binary ----
FROM rust-builder AS notifier-build
WORKDIR /app
COPY offchain/notifier/src offchain/notifier/src
RUN cargo build -p notifier --release


# ---- Build keeper binary ----
FROM rust-builder AS keeper-build
WORKDIR /app
COPY offchain/keeper/src offchain/keeper/src
RUN cargo build -p keeper --release


# ---- Node builder untuk Frontend ----
FROM node:20-alpine AS frontend-build
WORKDIR /app

# Copy deps dahulu → cache lebih awet
COPY frontend/package.json frontend/package-lock.json ./
RUN --mount=type=cache,target=/root/.npm npm ci --omit=dev

# Copy source termasuk index.html
COPY frontend/ ./
RUN npm run build


# ---- Runtime image: Backend ----
FROM debian:bookworm-slim AS backend
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=backend-build /app/target/release/backend /usr/local/bin/backend
ENV RUST_LOG=info
EXPOSE 8080
CMD ["/usr/local/bin/backend"]


# ---- Runtime image: Indexer ----
FROM debian:bookworm-slim AS indexer
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=indexer-build /app/target/release/indexer /usr/local/bin/indexer
ENV RUST_LOG=info
CMD ["/usr/local/bin/indexer"]


# ---- Runtime image: Notifier ----
FROM debian:bookworm-slim AS notifier
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=notifier-build /app/target/release/notifier /usr/local/bin/notifier
ENV RUST_LOG=info
CMD ["/usr/local/bin/notifier"]


# ---- Runtime image: Keeper ----
FROM debian:bookworm-slim AS keeper
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=keeper-build /app/target/release/keeper /usr/local/bin/keeper
ENV RUST_LOG=info
CMD ["/usr/local/bin/keeper"]


# ---- Runtime image: Frontend (Nginx SPA) ----
FROM nginx:alpine AS frontend
WORKDIR /usr/share/nginx/html
COPY --from=frontend-build /app/dist ./
# SPA fallback utk vue-router history mode
RUN printf 'server {\n\
  listen 80;\n\
  root /usr/share/nginx/html;\n\
  location / {\n\
    try_files $uri $uri/ /index.html;\n\
  }\n\
}\n' > /etc/nginx/conf.d/default.conf
EXPOSE 80
