# voting-dapp/offchain/indexer/Dockerfile

FROM rust:1.83-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config libssl-dev build-essential ca-certificates && \
  rm -rf /var/lib/apt/lists/*

# Manifest + cache deps (tanpa src dulu)
COPY offchain/indexer/Cargo.toml offchain/indexer/Cargo.toml
RUN mkdir -p offchain/indexer/src && echo "fn main(){}" > offchain/indexer/src/main.rs && \
    cargo fetch --manifest-path offchain/indexer/Cargo.toml || true

# Copy source asli & install ke /out
COPY offchain/indexer/src offchain/indexer/src
RUN cargo install --path offchain/indexer --locked --root /out && \
    ls -lah /out/bin

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# Copy semua bin yang dihasilkan (biasanya 1 file)
COPY --from=builder /out/bin/* /usr/local/bin/

ENV RUST_LOG=info
# pastikan ini sesuai nama biner sebenarnya (lihat output ls di atas)
CMD ["/usr/local/bin/indexer"]
