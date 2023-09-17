
FROM rust:bookworm as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN cargo build --release --verbose && mv ./target/release/miaostay-cdn /miaostay-cdn

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /miaostay-cdn /miaostay-cdn

ENTRYPOINT ["/miaostay-cdn", "-C", "/Config.toml"]
