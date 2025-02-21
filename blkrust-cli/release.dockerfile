# Build Stage
FROM rust:1-alpine3.19 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src
RUN apk add --no-cache build-base
RUN cargo build --release

# Runtime Stage
FROM alpine:latest
RUN apk add --no-cache util-linux
WORKDIR /app
COPY --from=builder /app/target/release/blkrust-cli ./
## Adding environment variable to enable debug logs
ENV BLKRS_DEBUG=true
CMD ["./blkrust-cli","info" ,"vda1"]