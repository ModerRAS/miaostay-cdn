
FROM rust:bookworm as builder

WORKDIR /usr/src/app
RUN apt-get install -y pkg-config libssl-dev
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN cargo build --release --verbose && mv ./target/release/miaostay-cdn /miaostay-cdn

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl pkg-config libssl-dev

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /miaostay-cdn /miaostay-cdn

ENTRYPOINT ["/miaostay-cdn", "-C", "/Config.toml"]
