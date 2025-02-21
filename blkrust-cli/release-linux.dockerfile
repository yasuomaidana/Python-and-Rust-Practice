# Build Stage
FROM rust:1-alpine3.20 AS builder

RUN apk add --no-cache musl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo build --release --target aarch64-unknown-linux-musl

# Runtime Stage
FROM alpine:latest
RUN apk add --no-cache util-linux
WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/blkrust-cli ./
ENTRYPOINT ["sh"]  