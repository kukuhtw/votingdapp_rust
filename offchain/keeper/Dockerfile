# voting-dapp/offchain/keeper/Dockerfile

FROM rust:1.83-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config libssl-dev build-essential ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY offchain/keeper/Cargo.toml offchain/keeper/Cargo.toml
RUN mkdir -p offchain/keeper/src && echo "fn main(){}" > offchain/keeper/src/main.rs && \
    cargo fetch --manifest-path offchain/keeper/Cargo.toml || true

COPY offchain/keeper/src offchain/keeper/src
RUN cargo install --path offchain/keeper --locked --root /out && \
    ls -lah /out/bin

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /out/bin/* /usr/local/bin/

ENV RUST_LOG=info
CMD ["/usr/local/bin/keeper"]
