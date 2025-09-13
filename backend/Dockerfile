# voting-dapp/backend/Dockerfile

# ---- builder ----
FROM rust:1.83-slim AS builder
WORKDIR /app

# deps untuk build (openssl buat mysql/sqlx)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

# cache deps: WAJIB copy Cargo.toml agar cargo bisa jalan
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build --release || true

# source sebenernya
COPY src ./src
COPY migrations ./migrations

# build sekaligus “install” ke /out/bin/<nama-aktual-bin>
# (pakai --locked kalau kamu punya Cargo.lock di folder ini; kalau tidak ada, hapus --locked)
RUN cargo install --path . --root /out

# ---- runtime ----
FROM debian:bookworm-slim
WORKDIR /app

# runtime libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates tzdata libssl3 && \
    rm -rf /var/lib/apt/lists/*

# salin binary apapun namanya -> rename jadi /app/backend
COPY --from=builder /out/bin/* /app/backend

EXPOSE 8080
CMD ["/app/backend"]
