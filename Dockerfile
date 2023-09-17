FROM debian:bookworm-slim

ADD target/release/miaostay-cdn /

ENTRYPOINT ["/miaostay-cdn", "-C", "/Config.toml"]
