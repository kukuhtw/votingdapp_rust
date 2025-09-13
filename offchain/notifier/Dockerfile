# voting-dapp/offchain/notifier/Dockerfile
FROM rust:1.83-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config libssl-dev build-essential ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY offchain/notifier/Cargo.toml offchain/notifier/Cargo.toml
RUN mkdir -p offchain/notifier/src && echo "fn main(){}" > offchain/notifier/src/main.rs && \
    cargo fetch --manifest-path offchain/notifier/Cargo.toml || true

COPY offchain/notifier/src offchain/notifier/src
RUN cargo install --path offchain/notifier --locked --root /out && \
    ls -lah /out/bin

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /out/bin/* /usr/local/bin/

ENV RUST_LOG=info
CMD ["/usr/local/bin/notifier"]
